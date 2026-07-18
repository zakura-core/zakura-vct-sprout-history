#!/usr/bin/env python3
"""Split and verify Zakura's reviewed canonical Sprout-history artifact."""

import argparse
import hashlib
from pathlib import Path

EXPECTED_LEN = 71_710_871
EXPECTED_SHA256 = "abf89ec7b9eacbe7a259be891a17059496f2c7c7c2144d3babb34f85f8098832"
PART_SIZE = 8 * 1024 * 1024

parser = argparse.ArgumentParser()
parser.add_argument("artifact", type=Path)
parser.add_argument("--check", action="store_true", help="verify tracked parts without rewriting")
args = parser.parse_args()

data = args.artifact.read_bytes()
if len(data) != EXPECTED_LEN or hashlib.sha256(data).hexdigest() != EXPECTED_SHA256:
    raise SystemExit("artifact length or SHA-256 does not match the reviewed artifact")

repo = Path(__file__).resolve().parent.parent
for index, start in enumerate(range(0, len(data), PART_SIZE)):
    expected = data[start : start + PART_SIZE]
    output = repo / "crates" / f"part-{index:02d}" / "src" / "part.bin"
    if args.check:
        if output.read_bytes() != expected:
            raise SystemExit(f"tracked part differs: {output}")
    else:
        output.write_bytes(expected)
print(f"verified {len(data)} bytes ({EXPECTED_SHA256})")
