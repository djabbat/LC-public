import Config

config :ze_frontend, ZeFrontendWeb.Endpoint,
  debug_errors: true,
  code_reloader: true,
  check_origin: false,
  watchers: []

config :ze_frontend, :dev_routes, true

config :phoenix_live_reload,
  patterns: [
    ~r"priv/static/.*(js|css|png|jpeg|jpg|gif|svg)$",
    ~r"lib/ze_frontend_web/(controllers|live|components)/.*(ex|heex)$"
  ]