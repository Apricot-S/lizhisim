// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::TileKind;

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
}
