// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

pub type Tile = u8;
pub type TileCounts34 = [u8; 34];
pub type TileIds136 = Vec<u8>;

pub(crate) trait TileIds136Ext {
    fn to_ids136(&self) -> TileIds136;
}

impl TileIds136Ext for TileCounts34 {
    // Note: There is no distinction between red 5 and normal 5.
    fn to_ids136(&self) -> TileIds136 {
        self.iter()
            .enumerate()
            .flat_map(|(index, &count)| {
                let base_id = index as u8 * 4;
                (0..count).map(move |j| base_id + j)
            })
            .collect()
    }
}
