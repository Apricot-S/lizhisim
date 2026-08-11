// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bingpai::Bingpai;

pub struct Player {
    bingpai: Bingpai,
}

impl Player {
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be used by the Round qipai transition")
    )]
    pub(crate) fn from_qipai(bingpai: Bingpai) -> Self {
        Self { bingpai }
    }

    pub fn bingpai(&self) -> &Bingpai {
        &self.bingpai
    }
}

#[cfg(test)]
mod tests {
    use crate::bipai::Bipai;
    use crate::seat::FourPlayer;
    use crate::tile::TileKind;
    use crate::tile_set::TileSet;

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
    fn player_preserves_qipai_bingpai_counts() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let expected = *bingpai[0].counts();
        let player = Player::from_qipai(bingpai.into_iter().next().unwrap());

        assert_eq!(*player.bingpai().counts(), expected);
    }
}
