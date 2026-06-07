#!/usr/bin/env bash
# Run the same quality gates as .github/workflows/ci.yml (local dev).
# Usage: ./scripts/ci-local.sh
# Optional: git config core.hooksPath .githooks  (enables pre-push hook)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> cargo fmt --check"
cargo fmt --all -- --check

echo "==> cargo clippy"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> cargo test"
cargo test --workspace

if command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "==> coverage gate (shared-kernel >= 95%)"
  cargo llvm-cov -p shared-kernel --fail-under-lines 95
else
  echo "==> skip coverage (install: cargo install cargo-llvm-cov)"
fi

echo "CI local checks passed."
