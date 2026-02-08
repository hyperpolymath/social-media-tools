defmodule NujDashboardWeb.Router do
  use NujDashboardWeb, :router

  import NujDashboardWeb.UserAuth

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {NujDashboardWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
    plug :fetch_current_user
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", NujDashboardWeb do
    pipe_through :browser

    get "/", PageController, :home
  end

  # Authenticated routes
  scope "/", NujDashboardWeb do
    pipe_through [:browser, :require_authenticated_user]

    live_session :authenticated,
      on_mount: [{NujDashboardWeb.UserAuth, :ensure_authenticated}] do
      live "/dashboard", DashboardLive.Index, :index
      live "/platforms", PlatformLive.Index, :index
      live "/platforms/:id", PlatformLive.Show, :show
      live "/changes", ChangeLive.Index, :index
      live "/changes/:id", ChangeLive.Show, :show
      live "/guidance", GuidanceLive.Index, :index
      live "/guidance/:id", GuidanceLive.Show, :show
      live "/publications", PublicationLive.Index, :index
      live "/approvals", ApprovalLive.Index, :index
    end
  end

  # API routes
  scope "/api", NujDashboardWeb.API do
    pipe_through :api

    get "/health", HealthController, :check
    get "/platforms", PlatformController, :index
    get "/platforms/:id", PlatformController, :show
    get "/changes/recent", ChangeController, :recent
    get "/guidance/pending", GuidanceController, :pending
    post "/approvals/:id/approve", ApprovalController, :approve
    post "/approvals/:id/reject", ApprovalController, :reject
  end

  # Phoenix LiveDashboard (dev/monitoring)
  if Application.compile_env(:nuj_dashboard, :dev_routes) do
    import Phoenix.LiveDashboard.Router

    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard",
        metrics: NujDashboard.Telemetry,
        ecto_repos: [NujDashboard.Repo]
    end
  end
end
