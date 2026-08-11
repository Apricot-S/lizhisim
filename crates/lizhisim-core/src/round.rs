// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bipai::{Bipai, BipaiSpec, QipaiCompleted, QipaiPending};
use crate::player::Player;
use crate::seat::FourPlayer;
use crate::seat::Seat;

pub struct Round<P: BipaiSpec> {
    bipai: Bipai<P, QipaiPending>,
    zhuangjia: Seat<P>,
}

pub struct ZimoPendingRound {
    bipai: Bipai<FourPlayer, QipaiCompleted>,
    players: [Player<FourPlayer>; 4],
    zhuangjia: Seat<FourPlayer>,
}

impl<P: BipaiSpec> Round<P> {
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be called by the Round creation boundary")
    )]
    pub(crate) fn new(bipai: Bipai<P, QipaiPending>, zhuangjia: Seat<P>) -> Self {
        Self { bipai, zhuangjia }
    }

    pub fn bipai(&self) -> &Bipai<P, QipaiPending> {
        &self.bipai
    }

    pub fn zhuangjia(&self) -> &Seat<P> {
        &self.zhuangjia
    }
}

impl Round<FourPlayer> {
    pub fn qipai(self) -> ZimoPendingRound {
        let (bipai, [bingpai0, bingpai1, bingpai2, bingpai3]) = self.bipai.qipai();
        let [seat0, seat1, seat2, seat3] = Seat::<FourPlayer>::ALL;
        let players = [
            Player::from_qipai(seat0, bingpai0),
            Player::from_qipai(seat1, bingpai1),
            Player::from_qipai(seat2, bingpai2),
            Player::from_qipai(seat3, bingpai3),
        ];

        ZimoPendingRound {
            bipai,
            players,
            zhuangjia: self.zhuangjia,
        }
    }
}

impl ZimoPendingRound {
    pub fn bipai(&self) -> &Bipai<FourPlayer, QipaiCompleted> {
        &self.bipai
    }

    pub fn players(&self) -> &[Player<FourPlayer>; 4] {
        &self.players
    }

    pub fn zhuangjia(&self) -> &Seat<FourPlayer> {
        &self.zhuangjia
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
    fn qipai_pending_round_preserves_bipai_and_zhuangjia() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia);

        assert_eq!(
            (round.bipai().remaining_count(), round.zhuangjia()),
            (122, &zhuangjia)
        );
    }

    #[test]
    fn qipai_consumes_pending_round_and_returns_zimo_pending_round() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia);

        let round: ZimoPendingRound = round.qipai();

        assert_eq!(round.zhuangjia(), &zhuangjia);
    }
}
