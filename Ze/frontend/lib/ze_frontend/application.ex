defmodule ZeFrontend.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      ZeFrontendWeb.Telemetry,
      {Phoenix.PubSub, name: ZeFrontend.PubSub},
      ZeFrontendWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: ZeFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    ZeFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end