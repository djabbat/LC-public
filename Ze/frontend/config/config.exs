import Config

config :ze_frontend, ZeFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: ZeFrontendWeb.ErrorHTML, json: ZeFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: ZeFrontend.PubSub,
  live_view: [signing_salt: "O+Q1WgyV"]

config :ze_frontend, ZeFrontendWeb.BackendClient,
  base_url: System.get_env("BACKEND_URL", "http://localhost:3009"),
  timeout: 15_000,
  retry_delay: 200,
  max_retries: 3

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"