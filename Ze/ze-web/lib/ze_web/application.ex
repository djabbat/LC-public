defmodule ZeWeb.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Phoenix.PubSub, name: ZeWeb.PubSub},
      ZeWebWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: ZeWeb.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    ZeWebWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
