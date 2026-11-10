#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROMPT_PATH="${1:-$SCRIPT_DIR/prompts/briefer.md}"
OUTPUT_DIR="${2:-$SCRIPT_DIR/outputs}"
RUNVIL_DIR="${RUNVIL_DIR:-$HOME/Development/runvil}"

cp "$SCRIPT_DIR/verify-briefer.rs" "$RUNVIL_DIR/crates/harness/examples/verify-briefer.rs"
exec cargo run --manifest-path "$RUNVIL_DIR/Cargo.toml" -p harness --example verify-briefer -- "$PROMPT_PATH" "$OUTPUT_DIR"
