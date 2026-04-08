#!/usr/bin/env bash
# Run the BriefFlow backend test suite.
# Usage: ./run_tests.sh  (from the repo/ directory)
#
# DB-dependent tests panic when TEST_DATABASE_URL is absent.  This script
# automatically filters them out when no database is available, and runs
# them unconditionally when the variable is set.
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_DIR"

HAS_DB=false
[[ -n "${TEST_DATABASE_URL:-}" ]] || [[ -n "${DATABASE_URL:-}" ]] && HAS_DB=true

echo "============================================"
echo "  BriefFlow — Test Suite"
echo "============================================"
echo ""

# ── Step 1: type-check ──────────────────────────────────────────────────────
echo "[1/2] cargo check --package backend ..."
cargo check --package backend
echo "      OK"
echo ""

# ── Step 2: tests ───────────────────────────────────────────────────────────
if [[ "$HAS_DB" == "true" ]]; then
    echo "[2/2] cargo test --package backend  (ALL tests — DB available) ..."
    echo "      DB: ${TEST_DATABASE_URL:-${DATABASE_URL}}"
    echo ""
    cargo test --package backend -- --nocapture
else
    echo "[2/2] cargo test --package backend  (no-DB tests only) ..."
    echo "      DB tests excluded — set TEST_DATABASE_URL to enable"
    echo ""
    cargo test --package backend -- --nocapture \
        --skip login \
        --skip customer \
        --skip add_to_cart \
        --skip confirm_order \
        --skip scan_voucher
fi
echo ""

echo "============================================"
echo "  Tests passed."
echo "============================================"
