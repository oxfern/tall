#!/bin/sh
# Refresh repos/iroh-docs: every .md page listed in https://docs.iroh.computer/llms.txt,
# at the same relative paths (see repos/sources.json). Reference only; nothing here is built.
set -eu
cd "$(dirname "$0")/.."
out=repos/iroh-docs
base=https://docs.iroh.computer
mkdir -p "$out"
# The two browser examples are copied from n0-computer/iroh-examples; keep them.
find "$out" -name '*.md' -not -path "$out/examples/browser-*" -delete
curl -fsSL "$base/llms.txt" -o "$out/llms.txt"
grep -o "$base/[^) ]*\.md" "$out/llms.txt" | sort -u | while read -r url; do
  path=${url#"$base"/}
  mkdir -p "$out/$(dirname "$path")"
  curl -fsSL "$url" -o "$out/$path" || echo "missing: $url" >&2
done
echo "fetched $(find "$out" -name '*.md' | wc -l | tr -d ' ') pages on $(date -u +%F)"
