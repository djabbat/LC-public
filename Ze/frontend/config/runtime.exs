import Config

if config_env() == :prod do
  config :ze_frontend, ZeFrontendWeb.Endpoint,
    http: [port: String.to_integer(System.get_env("PORT") || "4009")],
    secret_key_base: System.fetch_env!("SECRET_KEY_BASE")
end