# AUDIT PACKET — LC_realtime

Path: `/home/oem/Desktop/LC/realtime`  Date: 2026-05-08

## Size & file counts
```
116K	/home/oem/Desktop/LC/realtime
```
**Extensions:** .ex=11, .exs=5, (noext)=1, .sh=1, .service=1
## Tree (depth=2, max 200 entries)
```
.
./deploy
./deploy/scripts
./deploy/systemd
./mix.exs
./lib
./lib/longevitycommon_web
./lib/longevitycommon_realtime
./Dockerfile
./config
./config/runtime.exs
./config/dev.exs
./config/prod.exs
./config/config.exs
```
## Detected stack: **Phoenix/Elixir**
## Core files

### `mix.exs` (1081 chars)
```exs
defmodule LCRealtime.MixProject do
  use Mix.Project

  def project do
    [
      app: :longevitycommon_realtime,
      version: "0.1.0",
      elixir: "~> 1.14",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps()
    ]
  end

  def application do
    [
      mod: {LCRealtime.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      {:phoenix, "~> 1.7.12"},
      {:phoenix_pubsub, "~> 2.1"},
      {:ecto_sql, "~> 3.11"},
      {:postgrex, ">= 0.0.0"},
      {:phoenix_html, "~> 4.1"},
      {:plug_cowboy, "~> 2.7"},
      {:jason, "~> 1.4"},
      {:joken, "~> 2.6"},        # JWT verification
      {:cors_plug, "~> 3.0"}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "ecto.setup"],
      "ecto.setup": ["ecto.create", "ecto.migrate"],
      "ecto.reset": ["ecto.drop", "ecto.setup"]
    ]
  end
end

```
### `Dockerfile` (616 chars)
```
FROM elixir:1.17-otp-27 AS builder
WORKDIR /app
ENV MIX_ENV=prod
RUN mix local.hex --force && mix local.rebar --force
COPY mix.exs mix.lock ./
COPY config ./config
RUN mix deps.get --only prod && mix deps.compile
COPY lib ./lib
RUN mix compile && mix release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libstdc++6 openssl ncurses-bin ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/_build/prod/rel/longevitycommon_realtime ./
EXPOSE 4500
ENV PORT=4500 PHX_HOST=0.0.0.0 PHX_SERVER=true
CMD ["bin/longevitycommon_realtime", "start"]

```
### code `lib/longevitycommon_realtime/application.ex`
```
defmodule LCRealtime.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      LCRealtime.Repo,
      {Phoenix.PubSub, name: LCRealtime.PubSub},
      LCRealtimeWeb.Endpoint,
      # Phase 4.5 (2026-05-08): postgres LISTEN/NOTIFY bridge from
      # Rust social-server (writes pg_notify) → Phoenix Channel.
      LCRealtime.FeedNotifier,
    ]

    opts = [strategy: :one_for_one, name: LCRealtime.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    LCRealtimeWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end

```
### code `lib/longevitycommon_web/router.ex`
```
defmodule LCRealtimeWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", LCRealtimeWeb do
    pipe_through :api
    get "/health", HealthController, :index
  end
end

```
### code `lib/longevitycommon_web/endpoint.ex`
```
defmodule LCRealtimeWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :longevitycommon_realtime

  socket "/socket", LCRealtimeWeb.UserSocket,
    websocket: true,
    longpoll: false

  plug CORSPlug, origin: "*"
  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Phoenix.json_library()

  plug Plug.MethodOverride
  plug Plug.Head
  plug LCRealtimeWeb.Router
end

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .ex | 11 | 8252 |
| .exs | 5 | 3269 |