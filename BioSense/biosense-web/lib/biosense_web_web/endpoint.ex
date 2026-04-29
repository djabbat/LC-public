defmodule BiosenseWebWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :biosense_web

  @session_options [
    store: :cookie,
    key: "_biosense_web_key",
    signing_salt: "BioSenseSalt2026Apr28RegenOK01padpad",
    same_site: "Lax"
  ]

  socket "/live", Phoenix.LiveView.Socket,
    websocket: [connect_info: [session: @session_options]],
    longpoll: [connect_info: [session: @session_options]]

  plug Plug.Static,
    at: "/",
    from: :biosense_web,
    gzip: false,
    only: BiosenseWebWeb.static_paths()

  if code_reloading? do
    socket "/phoenix/live_reload/socket", Phoenix.LiveReloader.Socket
    plug Phoenix.LiveReloader
    plug Phoenix.CodeReloader
  end

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Phoenix.json_library()

  plug Plug.MethodOverride
  plug Plug.Head
  plug Plug.Session, @session_options
  plug BiosenseWebWeb.Router
end
