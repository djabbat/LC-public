import Config

config :ze_web,
  generators: [timestamp_type: :utc_datetime],
  backend_url: System.get_env("ZE_BACKEND_URL") || "http://127.0.0.1:4001"

config :ze_web, ZeWebWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: ZeWeb.ErrorHTML, json: ZeWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: ZeWeb.PubSub,
  live_view: [signing_salt: "ZeViewSalt2026Apr28RegenerationOK01"]

config :phoenix, :json_library, Jason

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

import_config "#{config_env()}.exs"
