#!/usr/bin/env bash
# Запуск BioSense в dev: Rust backend (:4101) + Phoenix frontend (:4100).
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

echo "[run.sh] cargo run -p biosense-backend (:4101)…"
cargo run --release -p biosense-backend &
BACKEND_PID=$!

sleep 3

echo "[run.sh] mix phx.server (:4100)…"
( cd biosense-web && mix phx.server ) &
PHX_PID=$!

echo
echo "[run.sh] готово."
echo "  Phoenix (UI):     http://127.0.0.1:4100"
echo "  Rust backend API: http://127.0.0.1:4101/healthz"
echo "  Ctrl-C для остановки."
echo

wait
