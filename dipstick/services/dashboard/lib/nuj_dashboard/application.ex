defmodule NujDashboard.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      NujDashboard.Telemetry,
      NujDashboard.Repo,
      {DNSCluster, query: Application.get_env(:nuj_dashboard, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: NujDashboard.PubSub},
      {Finch, name: NujDashboard.Finch},
      {Redix, host: Application.get_env(:nuj_dashboard, :redis_host, "redis"), name: :redix},
      NujDashboardWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: NujDashboard.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    NujDashboardWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
