# Zakura VCT Sprout History

This repository publishes Zakura's canonical Mainnet VCT Sprout-history artifact as a family of crates.io packages. It keeps the artifact out of the primary Zakura source repository while preserving compile-time embedding and offline runtime use.

The facade crate is `zakura-vct-sprout-history`. Its ten exact-versioned part crates exist only to remain below crates.io's per-package size limit. Consumers should not depend on parts directly.

## Reviewed artifact

- Length: `71710871` bytes
- SHA-256: `abf89ec7b9eacbe7a259be891a17059496f2c7c7c2144d3babb34f85f8098832`
- Parts: seven 8 MiB parts, two 4 MiB parts, and one 4602007-byte remainder

## Verify

```sh
cargo test --workspace
./scripts/check-packages.sh
./scripts/split-artifact.py /path/to/mainnet-sprout-history.bin --check
```

## Update an artifact

Generate the contiguous artifact with Zakura's `generate-vct-sprout-artifact` tool. For a new reviewed artifact generation, update the expected length and digest in the split tool, facade, and consumer; split it here; and release a coordinated new major version of all packages. Never replace bytes in a published part version.

## Publish

Authenticate with crates.io, verify the commit and version, then run:

```sh
./scripts/publish.sh --execute
```

The script publishes all part crates first, waits for registry indexing, and publishes the facade last. After crates.io's initial five-new-crate burst, it backs off for the registry's ten-minute allowance refill between new package names. Tag the verified commit after publishing.
