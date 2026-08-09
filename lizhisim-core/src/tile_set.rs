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
    total_count: u16,
}

impl TileSet {
    pub const fn try_from_counts(counts: [u8; 37]) -> Result<Self, TileSetError> {
        let mut index = 0;
        let mut total_count = 0;
        while index < counts.len() {
            total_count += counts[index] as u16;
            index += 1;
        }

        Ok(Self {
            counts,
            total_count,
        })
    }

    pub const fn max_count(&self, tile_kind: TileKind) -> u8 {
        self.counts[tile_kind.index()]
    }

    pub const fn total_count(&self) -> u16 {
        self.total_count
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

    #[test]
    fn tile_set_total_count_is_sum_of_kind_counts() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 4;
        counts[TileKind::M2.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(tile_set.total_count(), 7);
    }
}
