defmodule Polygraph.Application do
  @moduledoc """
  The Polygraph Elixir application for distributed CRDT state management.
  """

  use Application

  @impl true
  def start(_type, _args) do
    children = [
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
