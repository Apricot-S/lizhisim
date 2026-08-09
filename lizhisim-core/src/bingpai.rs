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
}
