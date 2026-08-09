// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

//! Rule configuration and preset resolution for LizhiSim.

use thiserror::Error;

use lizhisim_core::{TileKind, TileSet, TileSetError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawRuleSpec {
    pub m0_count: u8,
    pub p0_count: u8,
    pub s0_count: u8,
}

#[derive(Debug, Error, PartialEq)]
pub enum RawRuleSpecError {
    #[error("{hong_baopai:?} count {actual_count} exceeds maximum {max_count}")]
    HongBaopaiCountOutOfRange {
        hong_baopai: TileKind,
        actual_count: u8,
        max_count: u8,
    },
    #[error("failed to resolve tile set: {0}")]
    TileSet(#[from] TileSetError),
}

impl RawRuleSpec {
    pub const fn new(m0_count: u8, p0_count: u8, s0_count: u8) -> Result<Self, RawRuleSpecError> {
        match validate_hong_baopai_count(TileKind::M0, m0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_hong_baopai_count(TileKind::P0, p0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_hong_baopai_count(TileKind::S0, s0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        Ok(Self {
            m0_count,
            p0_count,
            s0_count,
        })
    }

    pub fn resolve_tile_set(self) -> Result<TileSet, RawRuleSpecError> {
        let mut counts = [4; 37];
        counts[TileKind::M0.index()] = self.m0_count;
        counts[TileKind::M5.index()] = 4 - self.m0_count;
        counts[TileKind::P0.index()] = self.p0_count;
        counts[TileKind::P5.index()] = 4 - self.p0_count;
        counts[TileKind::S0.index()] = self.s0_count;
        counts[TileKind::S5.index()] = 4 - self.s0_count;

        match TileSet::try_from_counts(counts) {
            Ok(tile_set) => Ok(tile_set),
            Err(error) => Err(RawRuleSpecError::TileSet(error)),
        }
    }
}

const fn validate_hong_baopai_count(
    hong_baopai: TileKind,
    actual_count: u8,
) -> Result<(), RawRuleSpecError> {
    if actual_count > 4 {
        return Err(RawRuleSpecError::HongBaopaiCountOutOfRange {
            hong_baopai,
            actual_count,
            max_count: 4,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_rule_spec_accepts_each_m0_count_from_zero_through_four() {
        assert_eq!(
            [0, 1, 2, 3, 4].map(|m0_count| RawRuleSpec::new(m0_count, 0, 0)),
            [0, 1, 2, 3, 4].map(|m0_count| Ok(RawRuleSpec {
                m0_count,
                p0_count: 0,
                s0_count: 0,
            })),
        );
    }

    #[test]
    fn raw_rule_spec_accepts_each_p0_count_from_zero_through_four() {
        assert_eq!(
            [0, 1, 2, 3, 4].map(|p0_count| RawRuleSpec::new(0, p0_count, 0)),
            [0, 1, 2, 3, 4].map(|p0_count| Ok(RawRuleSpec {
                m0_count: 0,
                p0_count,
                s0_count: 0,
            })),
        );
    }

    #[test]
    fn raw_rule_spec_accepts_each_s0_count_from_zero_through_four() {
        assert_eq!(
            [0, 1, 2, 3, 4].map(|s0_count| RawRuleSpec::new(0, 0, s0_count)),
            [0, 1, 2, 3, 4].map(|s0_count| Ok(RawRuleSpec {
                m0_count: 0,
                p0_count: 0,
                s0_count,
            })),
        );
    }
}
