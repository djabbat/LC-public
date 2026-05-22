#!/bin/bash
# deploy.sh — install/refresh biosense-backend on the server.
# Idempotent: build → install systemd unit → enable → smoke.
#
# Per DEPLOY_CONVENTION.md (~/Desktop/LC/docs/).

set -euo pipefail

REPO_ROOT="${REPO_ROOT:-/home/jaba/web/longevitycommon}"
BACKEND_DIR="$REPO_ROOT/BioSense/backend"
UNIT_NAME="biosense-backend.service"
UNIT_SRC="$BACKEND_DIR/deploy/systemd/$UNIT_NAME"
UNIT_DST="/etc/systemd/system/$UNIT_NAME"
ENV_FILE="/etc/aim/biosense_backend.env"

echo "[1/5] building release binary…"
cd "$BACKEND_DIR"
cargo build --release

echo "[2/5] ensuring env file at $ENV_FILE…"
if [ ! -f "$ENV_FILE" ]; then
    sudo install -d -o root -g root -m 0755 /etc/aim
    sudo tee "$ENV_FILE" > /dev/null <<'EOF'
PORT=4502
HOST=127.0.0.1
RUST_LOG=info
EOF
    sudo chmod 0640 "$ENV_FILE"
    echo "  created $ENV_FILE"
fi

echo "[3/5] installing systemd unit…"
sudo install -m 0644 "$UNIT_SRC" "$UNIT_DST"
sudo systemctl daemon-reload

echo "[4/5] enable + start…"
sudo systemctl enable --now "$UNIT_NAME"

echo "[5/5] smoke /healthz…"
sleep 2
if curl -sf http://127.0.0.1:4502/healthz > /dev/null; then
    echo "  ✓ healthz OK"
    echo
    echo "Done. Verify with:"
    echo "  sudo systemctl status $UNIT_NAME"
    echo "  curl https://biosense.longevity.ge/api/healthz   # via nginx"
else
    echo "  ✗ healthz FAILED — check 'journalctl -u $UNIT_NAME -n 50'" >&2
    exit 1
fi
