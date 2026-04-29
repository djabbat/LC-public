import Config

config :biosense_web, BiosenseWebWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4102],
  secret_key_base: "TestOnlyBioSenseSecretKeyBase64Bytes_NotForProduction_2026Apr28_padpadpadpadpadpadpadpad",
  server: false

config :logger, level: :warning
