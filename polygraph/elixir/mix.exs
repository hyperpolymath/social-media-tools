defmodule Polygraph.MixProject do
  use Mix.Project

  def project do
    [
      app: :polygraph,
      version: "0.2.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {Polygraph.Application, []}
    ]
  end

  defp deps do
    [
      # Phoenix for WebSocket support
      {:phoenix, "~> 1.7"},
      {:phoenix_pubsub, "~> 2.1"},

      # CRDT library
      {:delta_crdt, "~> 0.6"},

      # Distributed Erlang
      {:libcluster, "~> 3.3"},

      # JSON
      {:jason, "~> 1.4"},

      # HTTP client
      {:req, "~> 0.4"},

      # Telemetry
      {:telemetry, "~> 1.2"},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"}
    ]
  end
end
