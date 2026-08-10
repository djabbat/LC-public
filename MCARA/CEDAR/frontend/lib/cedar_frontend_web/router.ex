defmodule CEDARFrontendWeb.Router do
  use CEDARFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {CEDARFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", CEDARFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/detail/:entity_type/:entity_id", DetailLive, :show
    live "/hsc-lineage", DetailLive, :hsc_lineage
    live "/sobol", DetailLive, :sobol
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard", metrics: CEDARFrontendWeb.Telemetry
    end
  end
end