// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use lizhisim_rules::ValidatedRuleSet;
use thiserror::Error;

use crate::tile::TileKind;

const MAX_COPIES_PER_TILE: u8 = 4;

#[derive(Debug, Error, PartialEq)]
pub enum TileSetError {
    #[error("tile kind {tile_kind:?} has {actual_count} copies, exceeding maximum {max_count}")]
    TileCountExceeded {
        tile_kind: TileKind,
        actual_count: u8,
        max_count: u8,
    },
    #[error(
        "combined count of {hong_baopai:?} and {base_tile:?} is {actual_count}, exceeding maximum {max_count}"
    )]
    CombinedFiveCountExceeded {
        hong_baopai: TileKind,
        base_tile: TileKind,
        actual_count: u8,
        max_count: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileSet {
    counts: [u8; 37],
    total_count: u8,
}

const fn validate_combined_five_count(
    counts: &[u8; 37],
    hong_baopai: TileKind,
    base_tile: TileKind,
) -> Result<(), TileSetError> {
    let actual_count = counts[hong_baopai.index()] + counts[base_tile.index()];
    if actual_count > MAX_COPIES_PER_TILE {
        return Err(TileSetError::CombinedFiveCountExceeded {
            hong_baopai,
            base_tile,
            actual_count,
            max_count: MAX_COPIES_PER_TILE,
        });
    }
    Ok(())
}

impl TileSet {
    pub const fn try_from_counts(counts: [u8; 37]) -> Result<Self, TileSetError> {
        let mut index = 0;
        let mut total_count = 0;
        while index < counts.len() {
            if counts[index] > MAX_COPIES_PER_TILE {
                return Err(TileSetError::TileCountExceeded {
                    tile_kind: TileKind::ALL[index],
                    actual_count: counts[index],
                    max_count: MAX_COPIES_PER_TILE,
                });
            }
            total_count += counts[index];
            index += 1;
        }

        match validate_combined_five_count(&counts, TileKind::M0, TileKind::M5) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_combined_five_count(&counts, TileKind::P0, TileKind::P5) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_combined_five_count(&counts, TileKind::S0, TileKind::S5) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }

        Ok(Self {
            counts,
            total_count,
        })
    }

    pub const fn max_count(&self, tile_kind: TileKind) -> u8 {
        self.counts[tile_kind.index()]
    }

    pub const fn total_count(&self) -> u8 {
        self.total_count
    }
}

impl TryFrom<&ValidatedRuleSet> for TileSet {
    type Error = TileSetError;

    fn try_from(rules: &ValidatedRuleSet) -> Result<Self, Self::Error> {
        let hong_baopai = rules.hong_baopai();
        let mut counts = [MAX_COPIES_PER_TILE; 37];
        counts[TileKind::M0.index()] = hong_baopai.m0_count;
        counts[TileKind::M5.index()] = MAX_COPIES_PER_TILE - hong_baopai.m0_count;
        counts[TileKind::P0.index()] = hong_baopai.p0_count;
        counts[TileKind::P5.index()] = MAX_COPIES_PER_TILE - hong_baopai.p0_count;
        counts[TileKind::S0.index()] = hong_baopai.s0_count;
        counts[TileKind::S5.index()] = MAX_COPIES_PER_TILE - hong_baopai.s0_count;
        Self::try_from_counts(counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_set_reports_max_count_for_tile_kind() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(tile_set.max_count(TileKind::M1), 3);
    }

    #[test]
    fn tile_set_total_count_is_sum_of_kind_counts() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 4;
        counts[TileKind::M2.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(tile_set.total_count(), 7);
    }

    #[test]
    fn tile_set_rejects_kind_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 5;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::TileCountExceeded {
                tile_kind: TileKind::M1,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn tile_set_rejects_kind_count_above_four_at_last_index() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 4;
        counts[TileKind::Z7.index()] = 5;

        assert!(TileSet::try_from_counts(counts).is_err());
    }

    #[test]
    fn tile_set_rejects_combined_m0_and_m5_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::M0.index()] = 2;
        counts[TileKind::M5.index()] = 3;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::M0,
                base_tile: TileKind::M5,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn tile_set_rejects_combined_p0_and_p5_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::P0.index()] = 2;
        counts[TileKind::P5.index()] = 3;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::P0,
                base_tile: TileKind::P5,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn tile_set_rejects_combined_s0_and_s5_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::S0.index()] = 2;
        counts[TileKind::S5.index()] = 3;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::S0,
                base_tile: TileKind::S5,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }
}

#[cfg(test)]
mod rule_tests {
    use lizhisim_rules::{HongBaopaiConfig, RuleSet};

    use super::*;

    fn raw(m0_count: u8, p0_count: u8, s0_count: u8) -> RuleSet {
        RuleSet {
            hong_baopai: HongBaopaiConfig {
                m0_count,
                p0_count,
                s0_count,
            },
        }
    }

    #[test]
    fn rule_set_resolves_zero_hong_baopai_to_four_base_fives() {
        let rule_set = ValidatedRuleSet::try_from(raw(0, 0, 0)).unwrap();
        let tile_set = TileSet::try_from(&rule_set).unwrap();

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
    fn rule_set_resolves_red_three_to_three_base_fives() {
        let tile_set =
            TileSet::try_from(&ValidatedRuleSet::try_from(raw(1, 1, 1)).unwrap()).unwrap();

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
    fn rule_set_resolves_mahjong_soul_four_player_red_three_to_136_tiles() {
        let tile_set =
            TileSet::try_from(&ValidatedRuleSet::try_from(raw(1, 1, 1)).unwrap()).unwrap();

        assert_eq!(tile_set.total_count(), 136);
    }

    #[test]
    fn rule_set_resolves_mahjong_soul_four_player_non_five_tiles_to_four() {
        let tile_set =
            TileSet::try_from(&ValidatedRuleSet::try_from(raw(1, 1, 1)).unwrap()).unwrap();
        let non_five_counts = TileKind::ALL[..34]
            .iter()
            .copied()
            .filter(|tile_kind| !matches!(tile_kind, TileKind::M5 | TileKind::P5 | TileKind::S5))
            .map(|tile_kind| tile_set.max_count(tile_kind))
            .collect::<Vec<_>>();

        assert_eq!(non_five_counts, vec![4; 31]);
    }

    #[test]
    fn rule_set_resolves_four_hong_baopai_to_zero_base_fives() {
        let tile_set =
            TileSet::try_from(&ValidatedRuleSet::try_from(raw(4, 4, 4)).unwrap()).unwrap();

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
