#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

packages=()
for number in $(seq 0 8); do
  index="$(printf '%02d' "$number")"
  packages+=("zakura-vct-sprout-history-part-${index}")
done

for package in "${packages[@]}"; do
  cargo package --locked --allow-dirty --no-verify -p "$package"
  archive="target/package/${package}-1.0.0.crate"
  bytes="$(wc -c < "$archive")"
  printf '%s: %d bytes\n' "$(basename "$archive")" "$bytes"
  if [ "$bytes" -ge 9000000 ]; then
    echo "ERROR: package exceeds conservative 9,000,000-byte limit" >&2
    exit 1
  fi
done

# Before the first release, Cargo cannot package the facade because its exact
# part dependencies do not exist in the registry. `publish.sh` packages and
# verifies the facade after crates.io indexes all nine parts.
