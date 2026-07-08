defmodule MCAOAFrontendWeb.Router do
  use MCAOAFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {MCAOAFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", MCAOAFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/counter/:counter_id", DetailLive, :show
    live "/counter-registry", DashboardLive, :registry
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard",
        metrics: MCAOAFrontendWeb.Telemetry,
        ecto_repos: []
    end
  end
end