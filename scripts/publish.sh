#!/usr/bin/env bash
set -euo pipefail

if [ "${1:-}" != "--execute" ]; then
  echo "usage: $0 --execute" >&2
  exit 2
fi

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
./scripts/check-packages.sh

for number in $(seq 0 8); do
  index="$(printf '%02d' "$number")"
  cargo publish --locked -p "zakura-vct-sprout-history-part-${index}"
done

for attempt in $(seq 1 20); do
  if cargo publish --locked -p zakura-vct-sprout-history; then
    exit 0
  fi
  if [ "$attempt" -eq 20 ]; then
    echo "facade publish failed after waiting for registry indexing" >&2
    exit 1
  fi
  sleep 15
done
