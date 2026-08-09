// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::TileKind;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TileSetError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileSet {
    counts: [u8; 37],
}

impl TileSet {
    pub const fn try_from_counts(counts: [u8; 37]) -> Result<Self, TileSetError> {
        Ok(Self { counts })
    }

    pub const fn max_count(&self, tile_kind: TileKind) -> u8 {
        self.counts[tile_kind.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physically_possible_counts_construct_tile_set() {
        assert!(TileSet::try_from_counts([0; 37]).is_ok());
    }

    #[test]
    fn tile_set_reports_max_count_for_tile_kind() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(tile_set.max_count(TileKind::M1), 3);
    }
}
