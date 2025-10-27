// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::tile::Tile;
use arrayvec::ArrayVec;
use rand::Rng;

pub(super) const MAX_TILE_COPIES: u8 = 4;
pub(super) const NUM_WANGPAI: usize = 14;
pub(super) const MAX_GANG_COUNT: usize = 4;
pub(super) const MAX_NUM_BAOPAI: usize = MAX_GANG_COUNT + 1;
pub(super) const NUM_HAND_TILES: usize = 13;

/// 壁牌: Wall.
pub(crate) trait Bipai {
    type Config;
    type Error;

    fn new(rng: &mut impl Rng, config: &Self::Config) -> Self;
    fn from_slice(bipai: &[u8], config: &Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn left_tile_count(&self) -> u8;
    fn baopai_count(&self) -> u8;
    fn baopai_indicators(&self) -> ArrayVec<Tile, MAX_NUM_BAOPAI>;
    fn libaopai_indicators(&self) -> ArrayVec<Tile, MAX_NUM_BAOPAI>;

    fn qipai(&self, player_index: usize) -> [Tile; NUM_HAND_TILES];
    fn zimo(&mut self) -> Tile;
    fn lingshangzimo(&mut self) -> Tile;
    fn kaigang(&mut self);
}
