// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

//! Rule configuration and preset resolution for LizhiSim.

use thiserror::Error;

use lizhisim_core::{TileKind, TileSet, TileSetError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawRuleSpec {
    pub m0_count: u8,
}

#[derive(Debug, Error, PartialEq)]
pub enum RawRuleSpecError {
    #[error("M0 count {actual_count} exceeds maximum {max_count}")]
    M0CountOutOfRange { actual_count: u8, max_count: u8 },
    #[error("failed to resolve tile set: {0}")]
    TileSet(#[from] TileSetError),
}

impl RawRuleSpec {
    pub const fn new(m0_count: u8) -> Result<Self, RawRuleSpecError> {
        if m0_count > 4 {
            return Err(RawRuleSpecError::M0CountOutOfRange {
                actual_count: m0_count,
                max_count: 4,
            });
        }
        Ok(Self { m0_count })
    }

    pub fn resolve_tile_set(self) -> Result<TileSet, RawRuleSpecError> {
        let mut counts = [4; 37];
        counts[TileKind::M0.index()] = self.m0_count;
        counts[TileKind::M5.index()] = 4 - self.m0_count;
        match TileSet::try_from_counts(counts) {
            Ok(tile_set) => Ok(tile_set),
            Err(error) => Err(RawRuleSpecError::TileSet(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_rule_spec_accepts_each_m0_count_from_zero_through_four() {
        assert_eq!(
            [0, 1, 2, 3, 4].map(RawRuleSpec::new),
            [0, 1, 2, 3, 4].map(|m0_count| Ok(RawRuleSpec { m0_count })),
        );
    }
}
