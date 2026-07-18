#!/usr/bin/env bash
set -euo pipefail

if [ "${1:-}" != "--execute" ]; then
  echo "usage: $0 --execute" >&2
  exit 2
fi

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

for command in cargo; do
  command -v "$command" >/dev/null || {
    echo "missing required command: $command" >&2
    exit 1
  }
done

./scripts/check-packages.sh

publish_package() {
  package="$1"
  version="1.0.0"

  for attempt in $(seq 1 12); do
    search_result="$(cargo search "$package" --limit 5)"
    if [[ "$search_result" == *"$package = \"$version\""* ]]; then
      echo "${package} ${version} is available at crates.io"
      return
    fi
    if CARGO_HTTP_MULTIPLEXING=false cargo publish --locked -p "$package"; then
      return
    fi
    if [ "$attempt" -eq 12 ]; then
      echo "${package} publish failed after 12 attempts" >&2
      exit 1
    fi
    echo "waiting 10 minutes for the crates.io new-crate allowance to refill"
    sleep 600
  done
}

for number in $(seq 0 9); do
  index="$(printf '%02d' "$number")"
  publish_package "zakura-vct-sprout-history-part-${index}"
done

publish_package zakura-vct-sprout-history
