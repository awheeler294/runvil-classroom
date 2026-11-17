#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROMPT_PATH="${1:-$SCRIPT_DIR/prompts/17.11.2026-nash.md}"
OUTPUT_DIR="${OUTPUT_DIR:-$SCRIPT_DIR/outputs}"

if [ "$#" -gt 0 ]; then
    shift
fi

exec cargo run --manifest-path "$SCRIPT_DIR/runner/Cargo.toml" -- "$PROMPT_PATH" "$OUTPUT_DIR" ${@+"$@"}
