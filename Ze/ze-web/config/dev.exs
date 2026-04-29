import Config

config :ze_web, ZeWebWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4000],
  check_origin: false,
  code_reloader: true,
  debug_errors: true,
  secret_key_base: "DevOnlySecretKeyBase64Bytes_VeryLong_NotForProduction_2026Apr28_padpadpadpadpadpadpad",
  watchers: [],
  live_reload: [
    patterns: [
      ~r"priv/static/.*(js|css|png|jpeg|jpg|gif|svg)$",
      ~r"lib/ze_web_web/(controllers|live|components)/.*(ex|heex)$"
    ]
  ]

config :ze_web, dev_routes: true

config :logger, :console, format: "[$level] $message\n"

config :phoenix, :stacktrace_depth, 20
config :phoenix, :plug_init_mode, :runtime
