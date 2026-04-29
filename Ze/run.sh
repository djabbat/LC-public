#!/usr/bin/env bash
# Запуск Ze в dev режиме: Rust backend (:4001) + Phoenix frontend (:4000) одновременно.
# Использование:  ./run.sh
# Остановка:      Ctrl-C один раз — оба процесса завершатся.
set -euo pipefail
cd "$(dirname "$0")"

cleanup() {
  echo
  echo "[run.sh] остановка фоновых процессов..."
  kill "${BACKEND_PID:-}" "${PHX_PID:-}" 2>/dev/null || true
  wait 2>/dev/null || true
  exit 0
}
trap cleanup INT TERM EXIT

echo "[run.sh] cargo run -p ze-backend (:4001)…"
cargo run --release -p ze-backend &
BACKEND_PID=$!

# Дать backend'у пару секунд подняться, чтобы Phoenix не плакал на старте
sleep 3

echo "[run.sh] mix phx.server (:4000)…"
( cd ze-web && mix phx.server ) &
PHX_PID=$!

echo
echo "[run.sh] готово."
echo "  Phoenix (UI):     http://127.0.0.1:4000"
echo "  Rust backend API: http://127.0.0.1:4001/healthz"
echo "  Ctrl-C для остановки."
echo

wait
