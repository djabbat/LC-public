#!/usr/bin/env bash
#
# Deploy app.longevity.ge native (no Docker).
#
# Replaces docker-compose-all.yml services: realtime, server, web.
# Postgres uses host's native postgresql@16-main.service (no separate container).
#
# Run as user `jaba` on the server. Requires sudo for /opt and /etc and
# nginx symlinking.
#
# Usage:
#   ./deploy-app-native.sh build     # build releases locally (rust + phoenix + react)
#   ./deploy-app-native.sh install   # install systemd units, copy releases to /opt
#   ./deploy-app-native.sh enable    # enable + start services
#   ./deploy-app-native.sh status    # show service + nginx status
#   ./deploy-app-native.sh           # all of the above, in order

set -euo pipefail

REPO_DIR="${HOME}/web/longevitycommon"
DEPLOY_DIR="${REPO_DIR}/deploy"
SYSTEMD_DIR="${DEPLOY_DIR}/systemd"
NGINX_DIR="${DEPLOY_DIR}/nginx"

step_build() {
  echo "==> Building Rust social server (release)..."
  cd "${REPO_DIR}/server"
  cargo build --release
  echo "    OK: target/release/longevitycommon-server"

  echo "==> Building Phoenix realtime release..."
  cd "${REPO_DIR}/realtime"
  MIX_ENV=prod mix deps.get --only prod
  MIX_ENV=prod mix compile
  MIX_ENV=prod mix release --overwrite
  echo "    OK: _build/prod/rel/longevitycommon_realtime"

  echo "==> Building React PWA (Vite production build)..."
  cd "${REPO_DIR}/web"
  npm ci
  npm run build
  echo "    OK: dist/"
}

step_install() {
  echo "==> Installing /opt/app-social ..."
  sudo mkdir -p /opt/app-social/bin
  sudo install -m 0755 "${REPO_DIR}/server/target/release/longevitycommon-server" \
    /opt/app-social/bin/longevitycommon-server
  sudo chown -R jaba:jaba /opt/app-social

  echo "==> Installing /opt/app-realtime ..."
  sudo rm -rf /opt/app-realtime
  sudo mkdir -p /opt/app-realtime
  sudo cp -a "${REPO_DIR}/realtime/_build/prod/rel/longevitycommon_realtime/." \
    /opt/app-realtime/
  sudo chown -R jaba:jaba /opt/app-realtime

  echo "==> Installing static SPA to /var/www/longevitycommon-web ..."
  sudo mkdir -p /var/www/longevitycommon-web
  sudo rsync -a --delete "${REPO_DIR}/web/dist/" /var/www/longevitycommon-web/
  sudo chown -R www-data:www-data /var/www/longevitycommon-web

  echo "==> Installing systemd units ..."
  sudo install -m 0644 "${SYSTEMD_DIR}/app-realtime.service" /etc/systemd/system/
  sudo install -m 0644 "${SYSTEMD_DIR}/app-social.service"   /etc/systemd/system/
  sudo systemctl daemon-reload

  echo "==> Ensuring /etc/app-{realtime,social}/.env exist (must be filled in) ..."
  for svc in realtime social; do
    if [[ ! -f "/etc/app-${svc}/.env" ]]; then
      sudo mkdir -p "/etc/app-${svc}"
      sudo install -m 0600 "${SYSTEMD_DIR}/app-${svc}.env.template" "/etc/app-${svc}/.env"
      sudo chown root:jaba "/etc/app-${svc}/.env"
      echo "    !! /etc/app-${svc}/.env was templated; FILL IN secrets before enabling!"
    fi
  done

  echo "==> Linking nginx vhost ..."
  sudo install -m 0644 "${NGINX_DIR}/app.longevity.ge.conf" \
    /etc/nginx/sites-available/app.longevity.ge.conf
  sudo ln -sf /etc/nginx/sites-available/app.longevity.ge.conf \
    /etc/nginx/sites-enabled/app.longevity.ge.conf
  sudo nginx -t
}

step_enable() {
  echo "==> Enabling + starting services ..."
  sudo systemctl enable --now app-realtime.service
  sudo systemctl enable --now app-social.service
  sudo systemctl reload nginx
  echo "    OK"
}

step_status() {
  systemctl status app-realtime.service --no-pager --lines=5 || true
  echo
  systemctl status app-social.service --no-pager --lines=5 || true
  echo
  curl -fsS http://127.0.0.1:4500/  >/dev/null && echo "realtime:4500 OK"  || echo "realtime:4500 FAIL"
  curl -fsS http://127.0.0.1:4600/  >/dev/null && echo "social:4600 OK"    || echo "social:4600 FAIL"
  curl -fsS https://app.longevity.ge/ -I 2>&1 | head -2
}

case "${1:-all}" in
  build)   step_build ;;
  install) step_install ;;
  enable)  step_enable ;;
  status)  step_status ;;
  all)     step_build && step_install && step_enable && step_status ;;
  *)       echo "Usage: $0 {build|install|enable|status|all}" && exit 2 ;;
esac
