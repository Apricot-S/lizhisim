// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

use crate::seat::FourPlayer;
use crate::tile::TileKind;
use crate::tile_set::TileSet;

mod private {
    pub trait Sealed {}
}

pub trait PlayerSet: private::Sealed {
    type BipaiTiles: AsRef<[TileKind]>;
}

impl private::Sealed for FourPlayer {}

impl PlayerSet for FourPlayer {
    type BipaiTiles = [TileKind; 136];
}

#[derive(Debug, Error, PartialEq)]
pub enum BipaiError {
    #[error("bipai tiles do not match the tile set")]
    TileSetMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bipai<P: PlayerSet> {
    tiles: P::BipaiTiles,
    cursor: usize,
}

impl Bipai<FourPlayer> {
    pub fn try_new(tiles: [TileKind; 136], tile_set: &TileSet) -> Result<Self, BipaiError> {
        let actual_counts = tiles.iter().fold([0usize; 37], |mut counts, tile_kind| {
            counts[tile_kind.index()] += 1;
            counts
        });

        let matches_tile_set = TileKind::ALL.iter().enumerate().all(|(index, tile_kind)| {
            actual_counts[index] == usize::from(tile_set.max_count(*tile_kind))
        });
        if !matches_tile_set {
            return Err(BipaiError::TileSetMismatch);
        }

        Ok(Self { tiles, cursor: 52 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn red_three_tiles() -> ([TileKind; 136], TileSet) {
        let tile_set = TileSet::red_three_four_player();

        let mut tiles = [TileKind::M1; 136];
        let mut cursor = 0;
        for tile_kind in TileKind::ALL {
            for _ in 0..tile_set.max_count(tile_kind) {
                tiles[cursor] = tile_kind;
                cursor += 1;
            }
        }
        (tiles, tile_set)
    }

    #[test]
    fn four_player_bipai_accepts_tiles_matching_tile_set() {
        let (tiles, tile_set) = red_three_tiles();

        assert!(Bipai::<FourPlayer>::try_new(tiles, &tile_set).is_ok());
    }
}
