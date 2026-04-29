defmodule ZeWebWeb.Router do
  use ZeWebWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {ZeWebWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  scope "/", ZeWebWeb do
    pipe_through :browser

    live "/", SimulatorLive, :index
    live "/about", AboutLive, :index
  end
end
