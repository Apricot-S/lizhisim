// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::tile::Tile;

pub(super) const MAX_TILE_COPIES: u8 = 4;
pub(super) const NUM_WANGPAI: usize = 14;
pub(super) const MAX_GANG_COUNT: usize = 4;
pub(super) const MAX_NUM_BAOPAI: usize = MAX_GANG_COUNT + 1;
pub(super) const NUM_HAND_TILES: usize = 13;

/// 壁牌: Wall.
pub(crate) trait Bipai {
    fn left_tile_count(&self) -> u8;
    fn baopai_count(&self) -> u8;
    fn baopai_indicators(&self) -> &[Tile];
    fn libaopai_indicators(&self) -> &[Tile];

    fn qipai(&self, player_index: usize) -> [Tile; NUM_HAND_TILES];
    fn zimo(&mut self) -> Tile;
    fn lingshangzimo(&mut self) -> Tile;
    fn kaigang(&mut self);
}
