//! Canonical Mainnet VCT Sprout-history artifact for Zakura.

use std::io::{self, Read};

use zakura_vct_sprout_history_part_00 as part_00;
use zakura_vct_sprout_history_part_01 as part_01;
use zakura_vct_sprout_history_part_02 as part_02;
use zakura_vct_sprout_history_part_03 as part_03;
use zakura_vct_sprout_history_part_04 as part_04;
use zakura_vct_sprout_history_part_05 as part_05;
use zakura_vct_sprout_history_part_06 as part_06;
use zakura_vct_sprout_history_part_07 as part_07;
use zakura_vct_sprout_history_part_08 as part_08;

/// Complete artifact length in bytes.
pub const TOTAL_LEN: usize = 71710871;
/// SHA-256 digest of the complete reviewed artifact.
pub const SHA256: [u8; 32] = [
    0xab, 0xf8, 0x9e, 0xc7, 0xb9, 0xea, 0xcb, 0xe7, 0xa2, 0x59, 0xbe, 0x89, 0x1a, 0x17, 0x05, 0x94,
    0x96, 0xf2, 0xc7, 0xc7, 0xc2, 0x14, 0x4d, 0x3b, 0xab, 0xb3, 0x4f, 0x85, 0xf8, 0x09, 0x88, 0x32,
];

/// Integrity metadata for one artifact part.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartMetadata {
    /// Byte offset in the complete artifact.
    pub offset: usize,
    /// Part length in bytes.
    pub len: usize,
    /// SHA-256 digest of the part.
    pub sha256: [u8; 32],
}

/// Ordered static artifact parts.
pub static PARTS: [&[u8]; 9] = [
    part_00::BYTES,
    part_01::BYTES,
    part_02::BYTES,
    part_03::BYTES,
    part_04::BYTES,
    part_05::BYTES,
    part_06::BYTES,
    part_07::BYTES,
    part_08::BYTES,
];

/// Metadata corresponding to [`PARTS`].
pub const PART_METADATA: [PartMetadata; 9] = [
    PartMetadata {
        offset: part_00::OFFSET,
        len: part_00::LEN,
        sha256: part_00::SHA256,
    },
    PartMetadata {
        offset: part_01::OFFSET,
        len: part_01::LEN,
        sha256: part_01::SHA256,
    },
    PartMetadata {
        offset: part_02::OFFSET,
        len: part_02::LEN,
        sha256: part_02::SHA256,
    },
    PartMetadata {
        offset: part_03::OFFSET,
        len: part_03::LEN,
        sha256: part_03::SHA256,
    },
    PartMetadata {
        offset: part_04::OFFSET,
        len: part_04::LEN,
        sha256: part_04::SHA256,
    },
    PartMetadata {
        offset: part_05::OFFSET,
        len: part_05::LEN,
        sha256: part_05::SHA256,
    },
    PartMetadata {
        offset: part_06::OFFSET,
        len: part_06::LEN,
        sha256: part_06::SHA256,
    },
    PartMetadata {
        offset: part_07::OFFSET,
        len: part_07::LEN,
        sha256: part_07::SHA256,
    },
    PartMetadata {
        offset: part_08::OFFSET,
        len: part_08::LEN,
        sha256: part_08::SHA256,
    },
];

/// Returns the artifact parts in canonical order.
pub fn chunks() -> impl ExactSizeIterator<Item = &'static [u8]> {
    PARTS.into_iter()
}

/// A resettable sequential reader over the statically embedded artifact parts.
#[derive(Clone, Debug, Default)]
pub struct Reader {
    part: usize,
    part_offset: usize,
    position: usize,
}

impl Reader {
    /// Creates a reader positioned at the start of the artifact.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the absolute byte position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the number of unread bytes.
    pub fn remaining(&self) -> usize {
        TOTAL_LEN.saturating_sub(self.position)
    }

    /// Repositions the reader at the start of the artifact.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Read for Reader {
    fn read(&mut self, mut output: &mut [u8]) -> io::Result<usize> {
        let requested = output.len();
        while !output.is_empty() && self.part < PARTS.len() {
            let source = &PARTS[self.part][self.part_offset..];
            let copied = source.len().min(output.len());
            output[..copied].copy_from_slice(&source[..copied]);
            self.part_offset += copied;
            self.position += copied;
            output = &mut output[copied..];
            if self.part_offset == PARTS[self.part].len() {
                self.part += 1;
                self.part_offset = 0;
            }
        }
        Ok(requested - output.len())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn metadata_and_part_digests_match() {
        let mut expected_offset = 0;
        for (part, metadata) in PARTS.iter().zip(PART_METADATA) {
            assert_eq!(metadata.offset, expected_offset);
            assert_eq!(metadata.len, part.len());
            assert_eq!(<[u8; 32]>::from(Sha256::digest(part)), metadata.sha256);
            expected_offset += part.len();
        }
        assert_eq!(expected_offset, TOTAL_LEN);
    }

    #[test]
    fn complete_digest_matches() {
        let mut digest = Sha256::new();
        for part in chunks() {
            digest.update(part);
        }
        assert_eq!(<[u8; 32]>::from(digest.finalize()), SHA256);
    }

    #[test]
    fn reader_crosses_every_part_boundary_and_resets() {
        let mut reader = Reader::new();
        let mut digest = Sha256::new();
        let mut buffer = [0; 65_537];
        loop {
            let read = reader.read(&mut buffer).expect("static bytes are readable");
            if read == 0 {
                break;
            }
            digest.update(&buffer[..read]);
        }
        assert_eq!(reader.position(), TOTAL_LEN);
        assert_eq!(reader.remaining(), 0);
        assert_eq!(<[u8; 32]>::from(digest.finalize()), SHA256);

        reader.reset();
        let mut magic = [0; 8];
        reader
            .read_exact(&mut magic)
            .expect("artifact contains magic");
        assert_eq!(&magic, b"ZKVCTSP1");
    }
}
