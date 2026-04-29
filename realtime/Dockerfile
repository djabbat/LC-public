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
