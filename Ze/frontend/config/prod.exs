import Config

config :ze_frontend, ZeFrontendWeb.Endpoint,
  url: [host: System.get_env("HOSTNAME", "localhost"), port: 4009],
  cache_static_manifest: "priv/static/cache_manifest.json",
  check_origin: ["//localhost", "//127.0.0.1"]

config :logger, level: :info