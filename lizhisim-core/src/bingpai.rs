// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::TileKind;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BingpaiError {
    #[error("tile kind is not present in bingpai: {tile_kind:?}")]
    TileNotPresent { tile_kind: TileKind },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bingpai {
    counts: [u8; 37],
}

impl Default for Bingpai {
    fn default() -> Self {
        Self { counts: [0; 37] }
    }
}

impl Bingpai {
    pub fn with_added(mut self, tile_kind: TileKind) -> Self {
        self.counts[tile_kind.index()] += 1;
        self
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
        assert_eq!(Bingpai::default().counts(), &[0; 37]);
    }

    #[test]
    fn adding_m1_to_empty_bingpai_has_one_m1() {
        assert_eq!(Bingpai::default().with_added(TileKind::M1).counts()[0], 1);
    }

    #[test]
    fn adding_m1_to_empty_bingpai_does_not_change_other_counts() {
        assert_eq!(
            &Bingpai::default().with_added(TileKind::M1).counts()[1..],
            &[0; 36],
        );
    }

    #[test]
    fn adding_same_tile_kind_four_times_has_count_four() {
        assert_eq!(
            Bingpai::default()
                .with_added(TileKind::M1)
                .with_added(TileKind::M1)
                .with_added(TileKind::M1)
                .with_added(TileKind::M1)
                .counts()[0],
            4,
        );
    }

    #[test]
    fn adding_m0_to_empty_bingpai_has_one_m0() {
        assert_eq!(
            Bingpai::default().with_added(TileKind::M0).counts()[TileKind::M0.index()],
            1,
        );
    }

    #[test]
    fn adding_m0_to_empty_bingpai_does_not_change_m5_count() {
        assert_eq!(
            Bingpai::default().with_added(TileKind::M0).counts()[TileKind::M5.index()],
            0,
        );
    }

    #[test]
    fn adding_m5_to_empty_bingpai_has_one_m5() {
        assert_eq!(
            Bingpai::default().with_added(TileKind::M5).counts()[TileKind::M5.index()],
            1,
        );
    }

    #[test]
    fn adding_m5_to_empty_bingpai_does_not_change_m0_count() {
        assert_eq!(
            Bingpai::default().with_added(TileKind::M5).counts()[TileKind::M0.index()],
            0,
        );
    }

    #[test]
    fn removing_present_tile_kind_decreases_its_count() {
        assert_eq!(
            Bingpai::default()
                .with_added(TileKind::M1)
                .with_removed(TileKind::M1)
                .unwrap()
                .counts()[TileKind::M1.index()],
            0,
        );
    }

    #[test]
    fn removing_absent_tile_kind_reports_the_tile_kind() {
        assert_eq!(
            Bingpai::default().with_removed(TileKind::M1),
            Err(BingpaiError::TileNotPresent {
                tile_kind: TileKind::M1,
            }),
        );
    }

    #[test]
    fn removing_absent_tile_kind_does_not_change_bingpai() {
        let bingpai = Bingpai::default();
        let _ = bingpai.clone().with_removed(TileKind::M1);

        assert_eq!(bingpai.counts(), &[0; 37]);
    }

    #[test]
    fn removing_m0_from_bingpai_holding_only_m5_does_not_substitute_m5() {
        assert_eq!(
            Bingpai::default()
                .with_added(TileKind::M5)
                .with_removed(TileKind::M0),
            Err(BingpaiError::TileNotPresent {
                tile_kind: TileKind::M0,
            }),
        );
    }

    #[test]
    fn removing_m5_from_bingpai_holding_only_m0_does_not_substitute_m0() {
        assert_eq!(
            Bingpai::default()
                .with_added(TileKind::M0)
                .with_removed(TileKind::M5),
            Err(BingpaiError::TileNotPresent {
                tile_kind: TileKind::M5,
            }),
        );
    }

    #[test]
    fn adding_m0_to_bingpai_holding_m5_does_not_substitute_m5() {
        let bingpai = Bingpai::default()
            .with_added(TileKind::M5)
            .with_added(TileKind::M0);

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
        let bingpai = Bingpai::default()
            .with_added(TileKind::M0)
            .with_added(TileKind::M5);

        assert_eq!(
            [
                bingpai.counts()[TileKind::M0.index()],
                bingpai.counts()[TileKind::M5.index()],
            ],
            [1, 1],
        );
    }
}
