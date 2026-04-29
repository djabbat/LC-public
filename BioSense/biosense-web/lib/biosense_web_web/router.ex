defmodule BiosenseWebWeb.Router do
  use BiosenseWebWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {BiosenseWebWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  scope "/", BiosenseWebWeb do
    pipe_through :browser

    live "/", SimulatorLive, :index
    live "/datasets", DatasetsLive, :index
    live "/about", AboutLive, :index
  end
end
