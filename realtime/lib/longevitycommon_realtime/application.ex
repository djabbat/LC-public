defmodule LCRealtime.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      LCRealtime.Repo,
      {Phoenix.PubSub, name: LCRealtime.PubSub},
      LCRealtimeWeb.Endpoint,
      # Phase 4.5 (2026-05-08): postgres LISTEN/NOTIFY bridge from
      # Rust social-server (writes pg_notify) → Phoenix Channel.
      LCRealtime.FeedNotifier,
    ]

    opts = [strategy: :one_for_one, name: LCRealtime.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    LCRealtimeWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
