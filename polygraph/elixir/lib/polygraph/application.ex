defmodule Polygraph.Application do
  @moduledoc """
  The Polygraph Elixir application for distributed CRDT state management.

  ## Storage architecture

  All persistent state is written to VeriSimDB (http://localhost:8080 by default,
  overridden via VERISIMDB_URL). Three collections are used:

  - `polygraph:graph`   — claim graph nodes and edges
  - `polygraph:cache`   — ephemeral lookup cache (replaces Redis)
  - `polygraph:history` — bitemporal event log (replaces XTDB)

  The `Polygraph.VeriSimDB` GenServer must be started before the `ClaimStore`
  because the ClaimStore delegates persistence to it.

  ## Legacy databases

  ArangoDB, Redis, and XTDB dependencies have been superseded by VeriSimDB.
  Their driver packages have been removed from mix.exs. If you need to run
  against the old stack for a migration window, set LEGACY_STORAGE=true.
  """

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # VeriSimDB client — MUST be first; other children depend on it
      {Polygraph.VeriSimDB, []},

      # CRDT store for distributed claim state
      {Polygraph.CRDT.ClaimStore, []},

      # PubSub for real-time updates
      {Phoenix.PubSub, name: Polygraph.PubSub},

      # Cluster supervision
      {Cluster.Supervisor, [topologies(), [name: Polygraph.ClusterSupervisor]]}
    ]

    opts = [strategy: :one_for_one, name: Polygraph.Supervisor]
    Supervisor.start_link(children, opts)
  end

  defp topologies do
    [
      polygraph: [
        strategy: Cluster.Strategy.Epmd,
        config: [
          hosts: [
            :"polygraph1@127.0.0.1",
            :"polygraph2@127.0.0.1",
            :"polygraph3@127.0.0.1"
          ]
        ]
      ]
    ]
  end
end
