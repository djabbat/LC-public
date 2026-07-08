defmodule MCAOAFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      MCAOAFrontendWeb.Telemetry,
      {Phoenix.PubSub, name: MCAOAFrontend.PubSub},
      MCAOAFrontendWeb.Endpoint,
      {Registry, keys: :unique, name: MCAOAFrontend.BackendClient.Registry}
    ]

    opts = [strategy: :one_for_one, name: MCAOAFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    MCAOAFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end