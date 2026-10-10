#!/bin/sh
# Copies the canonical datasets into this package. Usage: sync-data.sh [path/to/live-site/data]
set -e
here="$(cd "$(dirname "$0")/.." && pwd)"
site="${1:-$here/../../live-site/data}"
for f in nmn-human-trials.csv nmn-human-trials.json nmn-price-index-2026.csv nmn-price-index-2026.json datapackage.json; do
  cp "$site/$f" "$here/data/$f"
done
mkdir -p "$here/src/nmn_research_data/data"
cp "$here/data/"*.json "$here/src/nmn_research_data/data/"
rm -f "$here/src/nmn_research_data/data/datapackage.json"
for d in "$here/rust/data" "$here/go/nmndata/data"; do
  mkdir -p "$d"
  cp "$here/data/nmn-human-trials.json" "$here/data/nmn-price-index-2026.json" "$d/"
done
echo "synced from $site"
