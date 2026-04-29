import Config

config :ze_web, ZeWebWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base: "TestOnlySecretKeyBase64Bytes_VeryLong_NotForProduction_2026Apr28_padpadpadpadpadpadpad",
  server: false

config :logger, level: :warning
