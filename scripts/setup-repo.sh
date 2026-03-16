#!/bin/bash
# Set GitHub repo description and topics
# Run after: gh auth login
# Usage: bash scripts/setup-repo.sh

set -e

REPO="zheimr/cooper-station"

echo "Setting repo description..."
gh repo edit "$REPO" \
  --description "Open-source O'Neill Cylinder space habitat — Rust physics engine, 12 engineering modules, interactive dashboard" \
  --homepage "https://zheimr.github.io/cooper-station/"

echo "Setting repo topics..."
gh repo edit "$REPO" \
  --add-topic "space" \
  --add-topic "rust" \
  --add-topic "wasm" \
  --add-topic "physics" \
  --add-topic "engineering" \
  --add-topic "space-habitat" \
  --add-topic "oneill-cylinder" \
  --add-topic "simulation" \
  --add-topic "webassembly" \
  --add-topic "open-science"

echo ""
echo "Done! Repo metadata updated."
echo "View at: https://github.com/zheimr/cooper-station"
