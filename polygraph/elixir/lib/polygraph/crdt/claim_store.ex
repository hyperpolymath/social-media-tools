defmodule Polygraph.CRDT.ClaimStore do
  @moduledoc """
  CRDT-based distributed store for claim verification state.
  Uses Delta CRDTs for efficient conflict-free replication.
  """

  use GenServer
  alias DeltaCrdt.{AWLWWMap, CausalCrdt}

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  @impl true
  def init(_) do
    {:ok, crdt} = CausalCrdt.start_link(AWLWWMap)

    # Subscribe to cluster events
    :net_kernel.monitor_nodes(true)

    {:ok, %{crdt: crdt, peers: []}}
  end

  # Client API

  @doc """
  Store a claim verification result.
  """
  def put_verification(claim_id, verification) do
    GenServer.call(__MODULE__, {:put, claim_id, verification})
  end

  @doc """
  Get a claim verification result.
  """
  def get_verification(claim_id) do
    GenServer.call(__MODULE__, {:get, claim_id})
  end

  @doc """
  List all claims.
  """
  def list_claims do
    GenServer.call(__MODULE__, :list)
  end

  # Server callbacks

  @impl true
  def handle_call({:put, claim_id, verification}, _from, state) do
    CausalCrdt.put(state.crdt, claim_id, verification)

    # Broadcast to peers
    broadcast_delta(state)

    {:reply, :ok, state}
  end

  @impl true
  def handle_call({:get, claim_id}, _from, state) do
    result = CausalCrdt.read(state.crdt)
    |> Map.get(claim_id)

    {:reply, result, state}
  end

  @impl true
  def handle_call(:list, _from, state) do
    claims = CausalCrdt.read(state.crdt)
    |> Map.keys()

    {:reply, claims, state}
  end

  @impl true
  def handle_info({:nodeup, node}, state) do
    # New node joined - sync state
    send_full_state(node, state)
    {:noreply, %{state | peers: [node | state.peers]}}
  end

  @impl true
  def handle_info({:nodedown, node}, state) do
    {:noreply, %{state | peers: List.delete(state.peers, node)}}
  end

  @impl true
  def handle_info({:crdt_delta, delta}, state) do
    # Received delta from peer
    CausalCrdt.merge(state.crdt, delta)
    {:noreply, state}
  end

  # Private functions

  defp broadcast_delta(state) do
    delta = CausalCrdt.read(state.crdt, :delta)

    for peer <- state.peers do
      send({__MODULE__, peer}, {:crdt_delta, delta})
    end
  end

  defp send_full_state(node, state) do
    full_state = CausalCrdt.read(state.crdt)
    send({__MODULE__, node}, {:crdt_full, full_state})
  end
end
