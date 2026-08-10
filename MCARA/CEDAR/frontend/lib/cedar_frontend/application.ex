defmodule CEDARFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      CEDARFrontend.Repo,
      CEDARFrontend.PubSub,
      {Phoenix.PubSub, name: CEDARFrontend.PubSub},
      CEDARFrontendWeb.Endpoint,
      CEDARFrontendWeb.Telemetry,
      {Oban, Application.fetch_env!(:cedar_frontend, Oban)},
      {DynamicSupervisor, strategy: :one_for_one, name: CEDARFrontend.DynamicSupervisor}
    ]

    opts = [strategy: :one_for_one, name: CEDARFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    CEDARFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end