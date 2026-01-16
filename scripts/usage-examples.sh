#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if command -v cargo-fak >/dev/null 2>&1; then
  RUNNER=("cargo-fak")
  echo "Using installed cargo-fak binary."
else
  RUNNER=("cargo" "run" "--")
  echo "cargo-fak not found in PATH; using cargo run instead."
fi

tmp_dir="$(mktemp -d -t cargo-fak-examples 2>/dev/null || mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

echo "Temporary directory: $tmp_dir"
echo

echo "Example 1: dry-run + print path (no files created)"
CARGO_FAK_ADR_DIR="$tmp_dir" "${RUNNER[@]}" adr "Add authentication" --dry-run --print-path
echo

echo "Example 2: create ADR in temp dir"
"${RUNNER[@]}" adr "Add authentication" --dir "$tmp_dir" --print-path
echo

echo "Example 3: template file"
cat > "$tmp_dir/template.md" <<'EOF'
#### Context
Write context here.
#### Decision
Write decision here.
#### Status
accepted
#### Consequences
List consequences here.
EOF
"${RUNNER[@]}" adr "Template example" --dir "$tmp_dir" --template "$tmp_dir/template.md" --print-path
echo

echo "Example 4: inline template"
"${RUNNER[@]}" adr "Inline template" --dir "$tmp_dir" --template "#### Context" --print-path
echo

echo "Example 5: create RFC in temp dir via env var"
CARGO_FAK_RFC_DIR="$tmp_dir" "${RUNNER[@]}" rfc "Request process update" --print-path
echo

echo "Records created:"
ls -1 "$tmp_dir"
