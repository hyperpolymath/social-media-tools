# SPDX-License-Identifier: PMPL-1.0-or-later
# SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell
#
# VeriSimDB client for Polygraph.
#
# Replaces ArangoDB + Redis + XTDB with a single VeriSimDB instance.
# Collections used:
#
#   polygraph:graph   — graph nodes and edges (claim relationships)
#   polygraph:cache   — ephemeral lookup cache (formerly Redis)
#   polygraph:history — bitemporal write history (formerly XTDB)
#
# All writes go to VeriSimDB. Cache entries carry a :ttl_seconds field
# and are pruned lazily on read; history entries are append-only and
# timestamped with both valid-time and transaction-time.
#
# Configuration (environment variables):
#   VERISIMDB_URL   — Base URL, defaults to http://localhost:8080
#
# Usage:
#   # Start via supervision tree (see application.ex)
#   {:ok, _} = Polygraph.VeriSimDB.start_link([])
#
#   # Store a graph node
#   :ok = Polygraph.VeriSimDB.put_node("claim:abc123", %{text: "...", sources: []})
#
#   # Retrieve a graph node
#   {:ok, node} = Polygraph.VeriSimDB.get_node("claim:abc123")
#
#   # Store an edge between two nodes
#   :ok = Polygraph.VeriSimDB.put_edge("edge:abc-def", %{from: "claim:abc123", to: "claim:def456", rel: "contradicts"})
#
#   # Cache lookup (nil when absent or expired)
#   :ok = Polygraph.VeriSimDB.put_cache("url:https://...", %{resolved_at: "...", verdict: :true}, ttl_seconds: 3600)
#   {:ok, entry} | nil = Polygraph.VeriSimDB.get_cache("url:https://...")
#
#   # Append a bitemporal history entry
#   :ok = Polygraph.VeriSimDB.record_history("claim:abc123", :updated, %{...})
#
#   # Query history for an entity
#   {:ok, events} = Polygraph.VeriSimDB.get_history("claim:abc123")

defmodule Polygraph.VeriSimDB do
  @moduledoc """
  VeriSimDB HTTP client for Polygraph.

  Manages three collections:
  - `polygraph:graph`   — nodes and edges for the claim graph
  - `polygraph:cache`   — short-lived lookup cache (replaces Redis)
  - `polygraph:history` — append-only bitemporal log (replaces XTDB)

  Configured via `VERISIMDB_URL` env var (default: http://localhost:8080).
  """

  use GenServer
  require Logger

  ## ── Collection names ─────────────────────────────────────────────────────

  @collection_graph   "polygraph:graph"
  @collection_cache   "polygraph:cache"
  @collection_history "polygraph:history"

  ## ── API base path ─────────────────────────────────────────────────────────

  @api_prefix "/api/v1"

  ## ── Child spec / start_link ──────────────────────────────────────────────

  @doc "Start the VeriSimDB GenServer (call from supervision tree)."
  @spec start_link(keyword()) :: GenServer.on_start()
  def start_link(opts \\ []) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  ## ── Graph API ─────────────────────────────────────────────────────────────

  @doc """
  Store a graph node.

  `id` — unique node identifier (e.g. "claim:abc123")
  `data` — arbitrary map of node attributes
  """
  @spec put_node(String.t(), map()) :: :ok | {:error, term()}
  def put_node(id, data) when is_binary(id) and is_map(data) do
    GenServer.call(__MODULE__, {:put, @collection_graph, id, data})
  end

  @doc "Retrieve a graph node by id. Returns `{:ok, node}` or `{:error, :not_found}`."
  @spec get_node(String.t()) :: {:ok, map()} | {:error, term()}
  def get_node(id) when is_binary(id) do
    GenServer.call(__MODULE__, {:get, @collection_graph, id})
  end

  @doc "Delete a graph node by id."
  @spec delete_node(String.t()) :: :ok | {:error, term()}
  def delete_node(id) when is_binary(id) do
    GenServer.call(__MODULE__, {:delete, @collection_graph, id})
  end

  @doc """
  Store a directed edge.

  `id`   — unique edge identifier (e.g. "edge:abc-def")
  `data` — map with at minimum %{from: node_id, to: node_id, rel: string}
  """
  @spec put_edge(String.t(), map()) :: :ok | {:error, term()}
  def put_edge(id, data) when is_binary(id) and is_map(data) do
    GenServer.call(__MODULE__, {:put, @collection_graph, id, Map.put(data, :kind, "edge")})
  end

  @doc "Retrieve an edge by id."
  @spec get_edge(String.t()) :: {:ok, map()} | {:error, term()}
  def get_edge(id) when is_binary(id) do
    GenServer.call(__MODULE__, {:get, @collection_graph, id})
  end

  ## ── Cache API ─────────────────────────────────────────────────────────────

  @doc """
  Store a cache entry.

  Options:
  - `ttl_seconds` — seconds until the entry is considered stale (default: 3600)
  """
  @spec put_cache(String.t(), map(), keyword()) :: :ok | {:error, term()}
  def put_cache(key, data, opts \\ []) when is_binary(key) and is_map(data) do
    ttl = Keyword.get(opts, :ttl_seconds, 3600)
    expires_at = DateTime.utc_now() |> DateTime.add(ttl, :second) |> DateTime.to_iso8601()
    payload = Map.merge(data, %{_expires_at: expires_at, _cache_key: key})
    GenServer.call(__MODULE__, {:put, @collection_cache, key, payload})
  end

  @doc """
  Retrieve a cache entry. Returns `nil` when absent or expired.
  """
  @spec get_cache(String.t()) :: {:ok, map()} | nil | {:error, term()}
  def get_cache(key) when is_binary(key) do
    case GenServer.call(__MODULE__, {:get, @collection_cache, key}) do
      {:ok, entry} ->
        # Lazy TTL check: discard if past expiry
        case Map.get(entry, "_expires_at") || Map.get(entry, :_expires_at) do
          nil ->
            {:ok, entry}

          iso when is_binary(iso) ->
            with {:ok, expires_at, _} <- DateTime.from_iso8601(iso) do
              if DateTime.compare(DateTime.utc_now(), expires_at) == :gt do
                # Expired — fire-and-forget delete
                GenServer.cast(__MODULE__, {:delete, @collection_cache, key})
                nil
              else
                {:ok, entry}
              end
            else
              _ -> {:ok, entry}
            end
        end

      {:error, :not_found} ->
        nil

      error ->
        error
    end
  end

  @doc "Explicitly invalidate a cache entry."
  @spec invalidate_cache(String.t()) :: :ok | {:error, term()}
  def invalidate_cache(key) when is_binary(key) do
    GenServer.call(__MODULE__, {:delete, @collection_cache, key})
  end

  ## ── History API ───────────────────────────────────────────────────────────

  @doc """
  Append a bitemporal history event for an entity.

  `entity_id`  — the entity being tracked (e.g. "claim:abc123")
  `event_type` — atom describing the event (e.g. :created, :updated, :verdict_changed)
  `payload`    — additional data to record with the event
  """
  @spec record_history(String.t(), atom(), map()) :: :ok | {:error, term()}
  def record_history(entity_id, event_type, payload \\ %{})
      when is_binary(entity_id) and is_atom(event_type) and is_map(payload) do
    # Each history event has its own unique ID derived from entity + wall clock
    transaction_time = DateTime.utc_now()
    event_id = "#{entity_id}:#{event_type}:#{transaction_time |> DateTime.to_unix(:millisecond)}"

    event = Map.merge(payload, %{
      entity_id: entity_id,
      event_type: Atom.to_string(event_type),
      valid_time: transaction_time |> DateTime.to_iso8601(),
      transaction_time: transaction_time |> DateTime.to_iso8601()
    })

    GenServer.call(__MODULE__, {:put, @collection_history, event_id, event})
  end

  @doc """
  Retrieve all history events for `entity_id`, sorted by transaction_time ascending.

  Returns `{:ok, [event, ...]}` where each event is a map.
  """
  @spec get_history(String.t()) :: {:ok, [map()]} | {:error, term()}
  def get_history(entity_id) when is_binary(entity_id) do
    GenServer.call(__MODULE__, {:list_prefix, @collection_history, entity_id})
  end

  ## ── GenServer callbacks ──────────────────────────────────────────────────

  @impl GenServer
  def init(_opts) do
    base_url = System.get_env("VERISIMDB_URL", "http://localhost:8080")
    client = Req.new(base_url: base_url, retry: false, receive_timeout: 5_000)

    Logger.info("[Polygraph.VeriSimDB] Connected to VeriSimDB at #{base_url}")

    {:ok, %{client: client, base_url: base_url}}
  end

  @impl GenServer
  def handle_call({:put, collection, id, data}, _from, state) do
    path = "#{@api_prefix}/#{collection}/#{URI.encode(id)}"

    case Req.put(state.client, url: path, json: data) do
      {:ok, %{status: status}} when status in [200, 201, 204] ->
        {:reply, :ok, state}

      {:ok, %{status: status, body: body}} ->
        Logger.warning("[Polygraph.VeriSimDB] PUT #{path} returned #{status}: #{inspect(body)}")
        {:reply, {:error, {:http, status, body}}, state}

      {:error, reason} ->
        Logger.error("[Polygraph.VeriSimDB] PUT #{path} failed: #{inspect(reason)}")
        {:reply, {:error, reason}, state}
    end
  end

  @impl GenServer
  def handle_call({:get, collection, id}, _from, state) do
    path = "#{@api_prefix}/#{collection}/#{URI.encode(id)}"

    case Req.get(state.client, url: path) do
      {:ok, %{status: 200, body: body}} ->
        {:reply, {:ok, body}, state}

      {:ok, %{status: 404}} ->
        {:reply, {:error, :not_found}, state}

      {:ok, %{status: status, body: body}} ->
        Logger.warning("[Polygraph.VeriSimDB] GET #{path} returned #{status}: #{inspect(body)}")
        {:reply, {:error, {:http, status, body}}, state}

      {:error, reason} ->
        Logger.error("[Polygraph.VeriSimDB] GET #{path} failed: #{inspect(reason)}")
        {:reply, {:error, reason}, state}
    end
  end

  @impl GenServer
  def handle_call({:delete, collection, id}, _from, state) do
    path = "#{@api_prefix}/#{collection}/#{URI.encode(id)}"

    case Req.delete(state.client, url: path) do
      {:ok, %{status: status}} when status in [200, 204] ->
        {:reply, :ok, state}

      {:ok, %{status: 404}} ->
        # Already gone — treat as success (idempotent)
        {:reply, :ok, state}

      {:ok, %{status: status, body: body}} ->
        Logger.warning("[Polygraph.VeriSimDB] DELETE #{path} returned #{status}: #{inspect(body)}")
        {:reply, {:error, {:http, status, body}}, state}

      {:error, reason} ->
        Logger.error("[Polygraph.VeriSimDB] DELETE #{path} failed: #{inspect(reason)}")
        {:reply, {:error, reason}, state}
    end
  end

  @impl GenServer
  def handle_call({:list_prefix, collection, prefix}, _from, state) do
    # VeriSimDB list endpoint with optional prefix filter
    path = "#{@api_prefix}/#{collection}"

    case Req.get(state.client, url: path, params: [prefix: prefix]) do
      {:ok, %{status: 200, body: body}} when is_list(body) ->
        sorted =
          body
          |> Enum.sort_by(fn entry ->
            Map.get(entry, "transaction_time") || Map.get(entry, :transaction_time) || ""
          end)

        {:reply, {:ok, sorted}, state}

      {:ok, %{status: 200, body: body}} when is_map(body) ->
        # Some VeriSimDB versions return %{items: [...]}
        items = Map.get(body, "items") || Map.get(body, :items) || []
        {:reply, {:ok, items}, state}

      {:ok, %{status: status, body: body}} ->
        Logger.warning(
          "[Polygraph.VeriSimDB] LIST #{path}?prefix=#{prefix} returned #{status}: #{inspect(body)}"
        )
        {:reply, {:error, {:http, status, body}}, state}

      {:error, reason} ->
        Logger.error("[Polygraph.VeriSimDB] LIST #{path} failed: #{inspect(reason)}")
        {:reply, {:error, reason}, state}
    end
  end

  ## Cast: fire-and-forget delete (used for expired cache eviction)
  @impl GenServer
  def handle_cast({:delete, collection, id}, state) do
    path = "#{@api_prefix}/#{collection}/#{URI.encode(id)}"
    Req.delete(state.client, url: path)
    {:noreply, state}
  end
end
