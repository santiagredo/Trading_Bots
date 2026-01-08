#!/usr/bin/env bash
set -e

ENGINE_PATH="src-tauri/bin/application"
BACKEND_MANIFEST="../Backend/Cargo.toml"
BACKEND_BIN="../Backend/target/debug/application"
HEALTH_CHECK_URL="http://localhost:8082/health_check"

echo "Checking engine health..."

if curl -sf "$HEALTH_CHECK_URL" > /dev/null; then
  echo "Engine is running and healthy. Skipping binary copy."
else
  echo "Engine not responding. Building backend engine..."
  cargo build --manifest-path "$BACKEND_MANIFEST"

  echo "Copying engine binary..."
  # mkdir -p target/debug/bin
  cp "$BACKEND_BIN" "$ENGINE_PATH"
  chmod +x "$ENGINE_PATH"
fi

echo "Starting frontend dev server..."
npm run dev
