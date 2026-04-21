FROM elixir:1.14-alpine AS build

RUN apk add --no-cache build-base npm git

WORKDIR /app

RUN mix local.hex --force && \
    mix local.rebar --force

COPY mix.exs mix.lock ./
RUN mix deps.get --only prod

COPY config config
RUN mix deps.compile

COPY priv priv
COPY lib lib

RUN mix compile

RUN mix phx.digest

RUN MIX_ENV=prod mix release

FROM alpine:3.18 AS runtime

RUN apk add --no-cache openssl ncurses-libs

WORKDIR /app

COPY --from=build /app/_build/prod/rel/ze_frontend ./

ENV HOME=/app
ENV MIX_ENV=prod
ENV PORT=4009

EXPOSE 4009

CMD ["bin/ze_frontend", "start"]