import Config

config :longevitycommon_realtime, LCRealtimeWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4500],
  check_origin: false,
  code_reloader: true,
  debug_errors: true,
  secret_key_base: "dev_secret_key_base_change_in_production_min_64_chars_xxxxxxxxxxxxxxxxxx"

config :longevitycommon_realtime, LCRealtime.Repo,
  username: "postgres",
  password: "password",
  hostname: "localhost",
  database: "longevitycommon",
  stacktrace: true,
  show_sensitive_data_on_connection_error: true,
  pool_size: 10

config :logger, level: :debug
