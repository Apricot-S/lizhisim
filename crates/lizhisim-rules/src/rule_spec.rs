// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

use lizhisim_core::{TileKind, TileSet, TileSetError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HongBaopaiConfig {
    pub m0_count: u8,
    pub p0_count: u8,
    pub s0_count: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawRuleSpec {
    pub hong_baopai: HongBaopaiConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleSpec {
    hong_baopai: HongBaopaiConfig,
}

#[derive(Debug, Error, PartialEq)]
pub enum RuleSpecError {
    #[error("{hong_baopai:?} count {actual_count} exceeds maximum {max_count}")]
    HongBaopaiCountOutOfRange {
        hong_baopai: TileKind,
        actual_count: u8,
        max_count: u8,
    },
    #[error("failed to resolve tile set: {0}")]
    TileSet(#[from] TileSetError),
}

impl HongBaopaiConfig {
    const fn validate(&self) -> Result<(), RuleSpecError> {
        match validate_hong_baopai_count(TileKind::M0, self.m0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_hong_baopai_count(TileKind::P0, self.p0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_hong_baopai_count(TileKind::S0, self.s0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        Ok(())
    }
}

const fn validate_hong_baopai_count(
    hong_baopai: TileKind,
    actual_count: u8,
) -> Result<(), RuleSpecError> {
    if actual_count > 4 {
        return Err(RuleSpecError::HongBaopaiCountOutOfRange {
            hong_baopai,
            actual_count,
            max_count: 4,
        });
    }
    Ok(())
}

impl TryFrom<RawRuleSpec> for RuleSpec {
    type Error = RuleSpecError;

    fn try_from(raw: RawRuleSpec) -> Result<Self, Self::Error> {
        raw.hong_baopai.validate()?;

        Ok(Self {
            hong_baopai: raw.hong_baopai,
        })
    }
}

impl RuleSpec {
    pub fn resolve_tile_set(&self) -> Result<TileSet, RuleSpecError> {
        let mut counts = [4; 37];

        counts[TileKind::M0.index()] = self.hong_baopai.m0_count;
        counts[TileKind::M5.index()] = 4 - self.hong_baopai.m0_count;
        counts[TileKind::P0.index()] = self.hong_baopai.p0_count;
        counts[TileKind::P5.index()] = 4 - self.hong_baopai.p0_count;
        counts[TileKind::S0.index()] = self.hong_baopai.s0_count;
        counts[TileKind::S5.index()] = 4 - self.hong_baopai.s0_count;

        TileSet::try_from_counts(counts).map_err(RuleSpecError::TileSet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(m0_count: u8, p0_count: u8, s0_count: u8) -> RawRuleSpec {
        RawRuleSpec {
            hong_baopai: HongBaopaiConfig {
                m0_count,
                p0_count,
                s0_count,
            },
        }
    }

    #[test]
    fn rule_spec_accepts_each_m0_count_from_zero_through_four() {
        assert!(
            [0, 1, 2, 3, 4]
                .map(|count| RuleSpec::try_from(raw(count, 0, 0)).is_ok())
                .into_iter()
                .all(|accepted| accepted)
        );
    }

    #[test]
    fn rule_spec_accepts_each_p0_count_from_zero_through_four() {
        assert!(
            [0, 1, 2, 3, 4]
                .map(|count| RuleSpec::try_from(raw(0, count, 0)).is_ok())
                .into_iter()
                .all(|accepted| accepted)
        );
    }

    #[test]
    fn rule_spec_accepts_each_s0_count_from_zero_through_four() {
        assert!(
            [0, 1, 2, 3, 4]
                .map(|count| RuleSpec::try_from(raw(0, 0, count)).is_ok())
                .into_iter()
                .all(|accepted| accepted)
        );
    }

    #[test]
    fn rule_spec_rejects_m0_count_above_four() {
        assert_eq!(
            RuleSpec::try_from(raw(5, 0, 0)),
            Err(RuleSpecError::HongBaopaiCountOutOfRange {
                hong_baopai: TileKind::M0,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn rule_spec_rejects_p0_count_above_four() {
        assert_eq!(
            RuleSpec::try_from(raw(0, 5, 0)),
            Err(RuleSpecError::HongBaopaiCountOutOfRange {
                hong_baopai: TileKind::P0,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn rule_spec_rejects_s0_count_above_four() {
        assert_eq!(
            RuleSpec::try_from(raw(0, 0, 5)),
            Err(RuleSpecError::HongBaopaiCountOutOfRange {
                hong_baopai: TileKind::S0,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn rule_spec_resolves_zero_hong_baopai_to_four_base_fives() {
        let rule_spec = RuleSpec::try_from(raw(0, 0, 0)).unwrap();
        let tile_set = rule_spec.resolve_tile_set().unwrap();

        assert_eq!(
            [
                tile_set.max_count(TileKind::M0),
                tile_set.max_count(TileKind::M5),
                tile_set.max_count(TileKind::P0),
                tile_set.max_count(TileKind::P5),
                tile_set.max_count(TileKind::S0),
                tile_set.max_count(TileKind::S5),
            ],
            [0, 4, 0, 4, 0, 4],
        );
    }

    #[test]
    fn rule_spec_resolves_red_three_to_three_base_fives() {
        let tile_set = RuleSpec::try_from(raw(1, 1, 1))
            .unwrap()
            .resolve_tile_set()
            .unwrap();

        assert_eq!(
            [
                tile_set.max_count(TileKind::M0),
                tile_set.max_count(TileKind::M5),
                tile_set.max_count(TileKind::P0),
                tile_set.max_count(TileKind::P5),
                tile_set.max_count(TileKind::S0),
                tile_set.max_count(TileKind::S5),
            ],
            [1, 3, 1, 3, 1, 3],
        );
    }

    #[test]
    fn rule_spec_resolves_mahjong_soul_four_player_red_three_to_136_tiles() {
        let tile_set = RuleSpec::try_from(raw(1, 1, 1))
            .unwrap()
            .resolve_tile_set()
            .unwrap();

        assert_eq!(tile_set.total_count(), 136);
    }

    #[test]
    fn rule_spec_resolves_mahjong_soul_four_player_non_five_tiles_to_four() {
        let tile_set = RuleSpec::try_from(raw(1, 1, 1))
            .unwrap()
            .resolve_tile_set()
            .unwrap();
        let non_five_counts = TileKind::ALL[..34]
            .iter()
            .copied()
            .filter(|tile_kind| !matches!(tile_kind, TileKind::M5 | TileKind::P5 | TileKind::S5))
            .map(|tile_kind| tile_set.max_count(tile_kind))
            .collect::<Vec<_>>();

        assert_eq!(non_five_counts, vec![4; 31]);
    }

    #[test]
    fn rule_spec_resolves_four_hong_baopai_to_zero_base_fives() {
        let tile_set = RuleSpec::try_from(raw(4, 4, 4))
            .unwrap()
            .resolve_tile_set()
            .unwrap();

        assert_eq!(
            [
                tile_set.max_count(TileKind::M0),
                tile_set.max_count(TileKind::M5),
                tile_set.max_count(TileKind::P0),
                tile_set.max_count(TileKind::P5),
                tile_set.max_count(TileKind::S0),
                tile_set.max_count(TileKind::S5),
            ],
            [4, 0, 4, 0, 4, 0],
        );
    }
}
