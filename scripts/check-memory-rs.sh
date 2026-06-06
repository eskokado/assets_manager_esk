#!/usr/bin/env bash
# EP-000 placeholder — full LeakSanitizer gate at epic close (EP-003+).
set -euo pipefail

if [[ "${SKIP_MEMORY_CHECK:-}" == "1" ]]; then
  echo "SKIP_MEMORY_CHECK=1 — memory check skipped (bootstrap placeholder)"
  exit 0
fi

echo "==> Bootstrap memory check: cargo test --workspace"
cargo test --workspace
echo "PASS: bootstrap memory placeholder"
