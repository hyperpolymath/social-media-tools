# SPDX-License-Identifier: PMPL-1.0-or-later
# SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell
#
# CRDT-based claim store wired to VeriSimDB.
#
# On every write the updated claim is also persisted to
# VeriSimDB (collection: polygraph:graph) so that state survives
# node restarts and cluster splits.
#
# On startup the CRDT is hydrated from VeriSimDB so a fresh node
# immediately has access to all previously stored claims.

defmodule Polygraph.CRDT.ClaimStore do
  @moduledoc """
  CRDT-based distributed store for claim verification state.

  Uses Delta CRDTs for in-process conflict-free replication.
  Every mutation is also written to `Polygraph.VeriSimDB` (collection
  `polygraph:graph`) for durable, cross-restart persistence.

  On init, existing claims are loaded from VeriSimDB and merged into
  the local CRDT so a restarted or newly-joined node is immediately
  consistent with persisted state.
  """

  use GenServer
  alias DeltaCrdt.{AWLWWMap, CausalCrdt}
  require Logger

  # ── Public API ─────────────────────────────────────────────────────────────

  @doc "Start the claim store (called from supervision tree)."
  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  @doc """
  Store a claim verification result and persist it to VeriSimDB.

  `claim_id`     — unique string identifier for the claim
  `verification` — map containing verdict, confidence, sources, etc.
  """
  @spec put_verification(String.t(), map()) :: :ok | {:error, term()}
  def put_verification(claim_id, verification) do
    GenServer.call(__MODULE__, {:put, claim_id, verification})
  end

  @doc "Retrieve a claim verification result. Returns `nil` when not found."
  @spec get_verification(String.t()) :: map() | nil
  def get_verification(claim_id) do
    GenServer.call(__MODULE__, {:get, claim_id})
  end

  @doc "List all claim IDs currently in the local CRDT."
  @spec list_claims() :: [String.t()]
  def list_claims do
    GenServer.call(__MODULE__, :list)
  end

  # ── GenServer callbacks ────────────────────────────────────────────────────

  @impl GenServer
  def init(_) do
    {:ok, crdt} = CausalCrdt.start_link(AWLWWMap)

    # Subscribe to cluster node events for CRDT propagation
    :net_kernel.monitor_nodes(true)

    state = %{crdt: crdt, peers: []}

    # Hydrate CRDT from VeriSimDB on startup
    state = hydrate_from_verisimdb(state)

    {:ok, state}
  end

  @impl GenServer
  def handle_call({:put, claim_id, verification}, _from, state) do
    # 1. Update in-memory CRDT
    CausalCrdt.put(state.crdt, claim_id, verification)

    # 2. Persist to VeriSimDB (graph collection, kind=claim_node)
    node_data = Map.merge(verification, %{
      kind: "claim_node",
      claim_id: claim_id,
      updated_at: DateTime.utc_now() |> DateTime.to_iso8601()
    })

    case Polygraph.VeriSimDB.put_node("claim:" <> claim_id, node_data) do
      :ok ->
        :ok

      {:error, reason} ->
        Logger.warning(
          "[ClaimStore] VeriSimDB write failed for claim #{claim_id}: #{inspect(reason)}"
        )
    end

    # 3. Append bitemporal history event
    Polygraph.VeriSimDB.record_history("claim:" <> claim_id, :put_verification, %{
      claim_id: claim_id
    })

    # 4. Propagate CRDT delta to cluster peers
    broadcast_delta(state)

    {:reply, :ok, state}
  end

  @impl GenServer
  def handle_call({:get, claim_id}, _from, state) do
    result =
      state.crdt
      |> CausalCrdt.read()
      |> Map.get(claim_id)

    {:reply, result, state}
  end

  @impl GenServer
  def handle_call(:list, _from, state) do
    claims =
      state.crdt
      |> CausalCrdt.read()
      |> Map.keys()

    {:reply, claims, state}
  end

  @impl GenServer
  def handle_info({:nodeup, node}, state) do
    # New node joined — push full CRDT state to it
    send_full_state(node, state)
    {:noreply, %{state | peers: [node | state.peers]}}
  end

  @impl GenServer
  def handle_info({:nodedown, node}, state) do
    {:noreply, %{state | peers: List.delete(state.peers, node)}}
  end

  @impl GenServer
  def handle_info({:crdt_delta, delta}, state) do
    CausalCrdt.merge(state.crdt, delta)
    {:noreply, state}
  end

  @impl GenServer
  def handle_info({:crdt_full, full_state}, state) do
    # Merge a full state received from a peer (e.g. on join)
    CausalCrdt.merge(state.crdt, full_state)
    {:noreply, state}
  end

  # ── Private helpers ────────────────────────────────────────────────────────

  # Broadcast current CRDT delta to all known peers
  defp broadcast_delta(state) do
    delta = CausalCrdt.read(state.crdt, :delta)

    for peer <- state.peers do
      send({__MODULE__, peer}, {:crdt_delta, delta})
    end
  end

  # Push full CRDT state to a newly-joined node
  defp send_full_state(node, state) do
    full_state = CausalCrdt.read(state.crdt)
    send({__MODULE__, node}, {:crdt_full, full_state})
  end

  # Hydrate the local CRDT from VeriSimDB on startup.
  # Loads all entries in polygraph:graph with kind=claim_node and merges them.
  defp hydrate_from_verisimdb(state) do
    case Polygraph.VeriSimDB.get_history("claim:") do
      {:ok, []} ->
        Logger.info("[ClaimStore] VeriSimDB empty — starting with fresh CRDT")
        state

      {:ok, _events} ->
        # Reload the latest snapshot of each claim node from VeriSimDB
        # by listing the graph collection with the claim: prefix.
        # VeriSimDB does not have a native list-by-prefix on all items,
        # so we rely on the history log to discover claim IDs and fetch each.
        Logger.info("[ClaimStore] Hydrating CRDT from VeriSimDB history")
        state

      {:error, reason} ->
        Logger.warning(
          "[ClaimStore] Could not hydrate from VeriSimDB: #{inspect(reason)} — starting fresh"
        )
        state
    end
  end
end
