#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments (must contain benchmark-name)
# - SIZES_JSON: output JSON path

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_JSON="${SIZES_JSON:-}"

: "${STATE_JSON:?STATE_JSON is required}"
: "${SIZES_JSON:?SIZES_JSON is required}"

# Barretenberg CRS directory
BB_CRS_DIR="${HOME}/.bb-crs"

# Clear the CRS cache directory
bb_clear_crs() {
  if [[ -d "$BB_CRS_DIR" ]]; then
    rm -rf "$BB_CRS_DIR"
    echo "Cleared CRS cache: $BB_CRS_DIR" >&2
  fi
}

# Measure total size of CRS files in bytes
# Returns 0 if directory doesn't exist
bb_measure_crs_size() {
  if [[ -d "$BB_CRS_DIR" ]]; then
    du -sk "$BB_CRS_DIR" 2>/dev/null | awk '{print $1 * 1024}'
  else
    echo "0"
  fi
}

bb_write_sizes_and_constraints() {
  # Args:
  #   $1 = target name (e.g., sha256, ecdsa)
  #   $2 = artifact basename (e.g., sha256.json, ecdsa.json)
  #   $3 = STATE_JSON path
  #   $4 = OUT_JSON path
  #   $5 = SYSTEM_DIR (directory containing system scripts)
  #   $6 = CRS size in bytes
  local TARGET_NAME="$1"
  local ARTIFACT_BASENAME="$2"
  local STATE_JSON_PATH="$3"
  local OUT_JSON_PATH="$4"
  local SYSTEM_DIR="$5"
  local CRS_SIZE_BYTES="${6:-0}"

  local WORKSPACE_ROOT_PATH proof_path proof_size_bytes CIRCUIT_PATH circuit_size preprocessing_size_bytes

  WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON_PATH")
  proof_path="${WORKSPACE_ROOT_PATH}/target/proof"

  if [[ ! -f "$proof_path" ]]; then
    echo "proof not found for Noir size measurement" >&2
    return 1
  fi

  proof_size_bytes=$(stat -f %z "$proof_path" 2>/dev/null || stat -c %s "$proof_path")

  CIRCUIT_PATH=$(jq -r '."circuit-path"' "$STATE_JSON_PATH")
  circuit_size=$(stat -f %z "$CIRCUIT_PATH" 2>/dev/null || stat -c %s "$CIRCUIT_PATH")
  preprocessing_size_bytes=$(( circuit_size + CRS_SIZE_BYTES ))

  local json_output
  json_output=$(jq -n \
    --argjson proof_size "$proof_size_bytes" \
    --argjson preprocessing_size "$preprocessing_size_bytes" \
    '{proof_size: $proof_size, preprocessing_size: $preprocessing_size}')

  echo "$json_output" > "$OUT_JSON_PATH"
  jq . "$OUT_JSON_PATH" || true

  if command -v noir-profiler >/dev/null 2>&1; then
    # Derive input size label from the output filename: <target>_<SIZE>_sizes.json
    local SIZE_LABEL
    SIZE_LABEL=$(basename "$OUT_JSON_PATH" | sed -E "s/^${TARGET_NAME}_([^_]+)_sizes\\.json$/\\1/")

    local CIRCUIT_SIZE
    CIRCUIT_SIZE=$(
      cd "$WORKSPACE_ROOT_PATH" && \
      { noir-profiler gates \
          --artifact-path ./target/"$ARTIFACT_BASENAME" \
          --backend-path bb \
          --output ./target \
          -- --include_gates_per_opcode 2>&1 || true; } \
        | sed -n 's/.*Circuit size: \([0-9][0-9]*\).*/\1/p' \
        | head -n1
    )

    if [[ -n "$CIRCUIT_SIZE" && -n "$SIZE_LABEL" ]]; then
      local CONSTRAINTS_JSON_PATH UPDATED_JSON
      CONSTRAINTS_JSON_PATH="${SYSTEM_DIR}/circuit_sizes.json"

      if [[ -f "$CONSTRAINTS_JSON_PATH" ]]; then
        UPDATED_JSON=$(jq \
          --arg target "$TARGET_NAME" \
          --arg size_key "$SIZE_LABEL" \
          --argjson size_val "$CIRCUIT_SIZE" \
          '(.[$target] //= {}) | .[$target][$size_key] = $size_val' \
          "$CONSTRAINTS_JSON_PATH")
      else
        UPDATED_JSON=$(jq -n \
          --arg target "$TARGET_NAME" \
          --arg size_key "$SIZE_LABEL" \
          --argjson size_val "$CIRCUIT_SIZE" \
          '{($target): {($size_key): $size_val}}')
      fi

      printf "%s\n" "$UPDATED_JSON" > "$CONSTRAINTS_JSON_PATH"
    fi
  else
    echo "Warning: noir-profiler not found; skipping circuit size generation" >&2
  fi
}

BENCHMARK_NAME=$(jq -r '."benchmark-name"' "$STATE_JSON")

bb_clear_crs
"$SCRIPT_DIR/prove.sh" >/dev/null 2>&1 || true
CRS_SIZE=$(bb_measure_crs_size)
bb_write_sizes_and_constraints "$BENCHMARK_NAME" "${BENCHMARK_NAME}.json" "$STATE_JSON" "$OUT_JSON" "$SCRIPT_DIR" "$CRS_SIZE"
