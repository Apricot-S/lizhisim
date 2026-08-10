// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use core::marker::PhantomData;

use thiserror::Error;

use crate::{FourPlayer, TileKind, TileSet};

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
    player_set: PhantomData<fn() -> P>,
}

impl Bipai<FourPlayer> {
    pub fn try_new(tiles: [TileKind; 136], tile_set: &TileSet) -> Result<Self, BipaiError> {
        let mut actual_counts = [0; 37];
        let mut index = 0;
        while index < tiles.len() {
            actual_counts[tiles[index].index()] += 1;
            index += 1;
        }

        let mut tile_index = 0;
        while tile_index < TileKind::ALL.len() {
            let tile_kind = TileKind::ALL[tile_index];
            if actual_counts[tile_index] != tile_set.max_count(tile_kind) {
                return Err(BipaiError::TileSetMismatch);
            }
            tile_index += 1;
        }

        Ok(Self {
            tiles,
            cursor: 52,
            player_set: PhantomData,
        })
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
