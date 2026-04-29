FROM elixir:1.17-otp-27 AS builder
WORKDIR /app
ENV MIX_ENV=prod
RUN mix local.hex --force && mix local.rebar --force
COPY mix.exs mix.lock ./
COPY config ./config
RUN mix deps.get --only prod && mix deps.compile
COPY lib ./lib
COPY priv ./priv
RUN mix compile && mix release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
        libstdc++6 openssl ncurses-bin locales ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    sed -i 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && locale-gen
ENV LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8
WORKDIR /app
COPY --from=builder /app/_build/prod/rel/ze_web ./
EXPOSE 4000
ENV PORT=4000 PHX_HOST=0.0.0.0 PHX_SERVER=true
CMD ["bin/ze_web", "start"]
