// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

use crate::tile::TileKind;
use crate::tile_set::TileSet;

#[derive(Debug, Error, PartialEq)]
pub enum BingpaiError {
    #[error("tile kind is not present in bingpai: {tile_kind:?}")]
    TileNotPresent { tile_kind: TileKind },
    #[error("tile kind exceeds its maximum count of {max_count}: {tile_kind:?}")]
    TileCountExceeded { tile_kind: TileKind, max_count: u8 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bingpai {
    counts: [u8; 37],
    tile_set: TileSet,
}

impl Bingpai {
    #[cfg(test)]
    pub(crate) const fn red_three_four_player() -> Self {
        Self {
            counts: [0; 37],
            tile_set: TileSet::red_three_four_player(),
        }
    }

    pub const fn new(tile_set: TileSet) -> Self {
        Self {
            counts: [0; 37],
            tile_set,
        }
    }

    pub fn with_added(mut self, tile_kind: TileKind) -> Result<Self, BingpaiError> {
        let count = &mut self.counts[tile_kind.index()];
        let max_count = self.tile_set.max_count(tile_kind);
        if *count >= max_count {
            return Err(BingpaiError::TileCountExceeded {
                tile_kind,
                max_count,
            });
        }
        *count += 1;
        Ok(self)
    }

    pub fn with_removed(mut self, tile_kind: TileKind) -> Result<Self, BingpaiError> {
        self.counts[tile_kind.index()] = self.counts[tile_kind.index()]
            .checked_sub(1)
            .ok_or(BingpaiError::TileNotPresent { tile_kind })?;
        Ok(self)
    }

    pub const fn counts(&self) -> &[u8; 37] {
        &self.counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bingpai_has_zero_of_every_tile_kind() {
        assert_eq!(Bingpai::red_three_four_player().counts(), &[0; 37]);
    }

    #[test]
    fn adding_m1_to_empty_bingpai_has_one_m1() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M1)
                .unwrap()
                .counts()[0],
            1
        );
    }

    #[test]
    fn adding_m1_to_empty_bingpai_does_not_change_other_counts() {
        assert_eq!(
            &Bingpai::red_three_four_player()
                .with_added(TileKind::M1)
                .unwrap()
                .counts()[1..],
            &[0; 36],
        );
    }

    #[test]
    fn adding_same_tile_kind_four_times_has_count_four() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M1)
                .unwrap()
                .with_added(TileKind::M1)
                .unwrap()
                .with_added(TileKind::M1)
                .unwrap()
                .with_added(TileKind::M1)
                .unwrap()
                .counts()[0],
            4,
        );
    }

    #[test]
    fn adding_m0_to_empty_bingpai_has_one_m0() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M0)
                .unwrap()
                .counts()[TileKind::M0.index()],
            1,
        );
    }

    #[test]
    fn adding_m0_to_empty_bingpai_does_not_change_m5_count() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M0)
                .unwrap()
                .counts()[TileKind::M5.index()],
            0,
        );
    }

    #[test]
    fn adding_m5_to_empty_bingpai_has_one_m5() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M5)
                .unwrap()
                .counts()[TileKind::M5.index()],
            1,
        );
    }

    #[test]
    fn adding_m5_to_empty_bingpai_does_not_change_m0_count() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M5)
                .unwrap()
                .counts()[TileKind::M0.index()],
            0,
        );
    }

    #[test]
    fn removing_present_tile_kind_decreases_its_count() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M1)
                .unwrap()
                .with_removed(TileKind::M1)
                .unwrap()
                .counts()[TileKind::M1.index()],
            0,
        );
    }

    #[test]
    fn removing_absent_tile_kind_reports_the_tile_kind() {
        assert_eq!(
            Bingpai::red_three_four_player().with_removed(TileKind::M1),
            Err(BingpaiError::TileNotPresent {
                tile_kind: TileKind::M1,
            }),
        );
    }

    #[test]
    fn removing_absent_tile_kind_does_not_change_bingpai() {
        let bingpai = Bingpai::red_three_four_player();
        let _ = bingpai.clone().with_removed(TileKind::M1);

        assert_eq!(bingpai.counts(), &[0; 37]);
    }

    #[test]
    fn removing_m0_from_bingpai_holding_only_m5_does_not_substitute_m5() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M5)
                .unwrap()
                .with_removed(TileKind::M0),
            Err(BingpaiError::TileNotPresent {
                tile_kind: TileKind::M0,
            }),
        );
    }

    #[test]
    fn removing_m5_from_bingpai_holding_only_m0_does_not_substitute_m0() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::M0)
                .unwrap()
                .with_removed(TileKind::M5),
            Err(BingpaiError::TileNotPresent {
                tile_kind: TileKind::M5,
            }),
        );
    }

    #[test]
    fn adding_m0_to_bingpai_holding_m5_does_not_substitute_m5() {
        let bingpai = Bingpai::red_three_four_player()
            .with_added(TileKind::M5)
            .unwrap()
            .with_added(TileKind::M0)
            .unwrap();

        assert_eq!(
            [
                bingpai.counts()[TileKind::M0.index()],
                bingpai.counts()[TileKind::M5.index()],
            ],
            [1, 1],
        );
    }

    #[test]
    fn adding_m5_to_bingpai_holding_m0_does_not_substitute_m0() {
        let bingpai = Bingpai::red_three_four_player()
            .with_added(TileKind::M0)
            .unwrap()
            .with_added(TileKind::M5)
            .unwrap();

        assert_eq!(
            [
                bingpai.counts()[TileKind::M0.index()],
                bingpai.counts()[TileKind::M5.index()],
            ],
            [1, 1],
        );
    }

    #[test]
    fn adding_p0_to_empty_bingpai_does_not_change_p5_count() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::P0)
                .unwrap()
                .counts()[TileKind::P5.index()],
            0,
        );
    }

    #[test]
    fn adding_s0_to_empty_bingpai_does_not_change_s5_count() {
        assert_eq!(
            Bingpai::red_three_four_player()
                .with_added(TileKind::S0)
                .unwrap()
                .counts()[TileKind::S5.index()],
            0,
        );
    }

    #[test]
    fn adding_fifth_copy_of_tile_kind_reports_count_exceeded() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 4;
        let bingpai = Bingpai::new(TileSet::try_from_counts(counts).unwrap())
            .with_added(TileKind::M1)
            .unwrap()
            .with_added(TileKind::M1)
            .unwrap()
            .with_added(TileKind::M1)
            .unwrap()
            .with_added(TileKind::M1)
            .unwrap();

        assert_eq!(
            bingpai.with_added(TileKind::M1),
            Err(BingpaiError::TileCountExceeded {
                tile_kind: TileKind::M1,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn adding_beyond_tile_set_limit_reports_configured_max_count() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 1;
        let bingpai = Bingpai::new(TileSet::try_from_counts(counts).unwrap())
            .with_added(TileKind::M1)
            .unwrap();

        assert_eq!(
            bingpai.with_added(TileKind::M1),
            Err(BingpaiError::TileCountExceeded {
                tile_kind: TileKind::M1,
                max_count: 1,
            }),
        );
    }

    #[test]
    fn adding_tile_kind_with_zero_tile_set_limit_is_rejected() {
        let counts = [0; 37];
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(
            Bingpai::new(tile_set).with_added(TileKind::M1),
            Err(BingpaiError::TileCountExceeded {
                tile_kind: TileKind::M1,
                max_count: 0,
            }),
        );
    }

    #[test]
    fn adding_red_tile_with_one_copy_tile_set_limit_allows_one_copy() {
        let mut counts = [0; 37];
        counts[TileKind::M0.index()] = 1;
        let bingpai = Bingpai::new(TileSet::try_from_counts(counts).unwrap())
            .with_added(TileKind::M0)
            .unwrap();

        assert_eq!(bingpai.counts()[TileKind::M0.index()], 1);
    }

    #[test]
    fn adding_base_five_with_one_red_tile_allows_three_copies() {
        let mut counts = [0; 37];
        counts[TileKind::M0.index()] = 1;
        counts[TileKind::M5.index()] = 3;
        let bingpai = Bingpai::new(TileSet::try_from_counts(counts).unwrap())
            .with_added(TileKind::M5)
            .unwrap()
            .with_added(TileKind::M5)
            .unwrap()
            .with_added(TileKind::M5)
            .unwrap();

        assert_eq!(bingpai.counts()[TileKind::M5.index()], 3);
    }

    #[test]
    fn adding_base_five_with_four_red_tiles_is_rejected() {
        let mut counts = [0; 37];
        counts[TileKind::M0.index()] = 4;
        let bingpai = Bingpai::new(TileSet::try_from_counts(counts).unwrap());

        assert_eq!(
            bingpai.with_added(TileKind::M5),
            Err(BingpaiError::TileCountExceeded {
                tile_kind: TileKind::M5,
                max_count: 0,
            }),
        );
    }

    #[test]
    fn failed_addition_does_not_change_original_bingpai() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 1;
        let bingpai = Bingpai::new(TileSet::try_from_counts(counts).unwrap())
            .with_added(TileKind::M1)
            .unwrap();
        let original = bingpai.clone();
        let _ = bingpai.with_added(TileKind::M1);

        assert_eq!(original.counts()[TileKind::M1.index()], 1);
    }
}
