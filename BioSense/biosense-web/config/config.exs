import Config

config :biosense_web,
  generators: [timestamp_type: :utc_datetime],
  backend_url: System.get_env("BIOSENSE_BACKEND_URL") || "http://127.0.0.1:4101"

config :biosense_web, BiosenseWebWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: BiosenseWebWeb.ErrorHTML, json: BiosenseWebWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: BiosenseWeb.PubSub,
  live_view: [signing_salt: "BioSenseSalt2026Apr28RegenOK01padpad"]

config :phoenix, :json_library, Jason

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

import_config "#{config_env()}.exs"
