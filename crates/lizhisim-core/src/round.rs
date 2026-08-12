// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bipai::{Bipai, BipaiError, QipaiCompleted, QipaiPending};
use crate::player::Player;
use crate::seat::FourPlayer;
use crate::seat::Seat;
use crate::tile::TileKind;

pub struct Round<P, State> {
    state: State,
    zhuangjia: Seat<P>,
}

struct FourPlayerRoundData {
    bipai: Bipai<FourPlayer, QipaiCompleted>,
    players: [Player<FourPlayer>; 4],
    actor: Seat<FourPlayer>,
}

pub struct FourPlayerZimoPending {
    data: FourPlayerRoundData,
}

pub struct FourPlayerZimoCompleted {
    data: FourPlayerRoundData,
    zimopai: TileKind,
}

impl Round<FourPlayer, FourPlayerZimoPending> {
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be called by the Round creation boundary")
    )]
    pub(crate) fn new(bipai: Bipai<FourPlayer, QipaiPending>, zhuangjia: Seat<FourPlayer>) -> Self {
        let (bipai, mut bingpai) = bipai.qipai();
        bingpai.rotate_right(zhuangjia.index());
        let [bingpai0, bingpai1, bingpai2, bingpai3] = bingpai;
        let [seat0, seat1, seat2, seat3] = Seat::<FourPlayer>::ALL;
        let players = [
            Player::from_qipai(seat0, bingpai0),
            Player::from_qipai(seat1, bingpai1),
            Player::from_qipai(seat2, bingpai2),
            Player::from_qipai(seat3, bingpai3),
        ];

        Self {
            state: FourPlayerZimoPending {
                data: FourPlayerRoundData {
                    bipai,
                    players,
                    actor: zhuangjia,
                },
            },
            zhuangjia,
        }
    }

    pub fn bipai(&self) -> &Bipai<FourPlayer, QipaiCompleted> {
        &self.state.data.bipai
    }

    pub fn players(&self) -> &[Player<FourPlayer>; 4] {
        &self.state.data.players
    }

    pub fn actor(&self) -> &Seat<FourPlayer> {
        &self.state.data.actor
    }

    pub fn zimo(self) -> Result<Round<FourPlayer, FourPlayerZimoCompleted>, BipaiError> {
        let FourPlayerRoundData {
            bipai,
            players,
            actor,
        } = self.state.data;
        let (bipai, zimopai) = bipai.zimo()?;
        let data = FourPlayerRoundData {
            bipai,
            players,
            actor,
        };

        Ok(Round {
            state: FourPlayerZimoCompleted { data, zimopai },
            zhuangjia: self.zhuangjia,
        })
    }
}

impl Round<FourPlayer, FourPlayerZimoCompleted> {
    pub fn actor(&self) -> &Seat<FourPlayer> {
        &self.state.data.actor
    }

    pub fn zimopai(&self) -> TileKind {
        self.state.zimopai
    }
}

impl<P, State> Round<P, State> {
    pub fn zhuangjia(&self) -> &Seat<P> {
        &self.zhuangjia
    }
}

#[cfg(test)]
mod tests {
    use crate::bipai::Bipai;
    use crate::seat::FourPlayer;
    use crate::seat::Seat;
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
    fn round_starts_with_four_players() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[0]);

        assert_eq!(
            round.players().each_ref().map(Player::seat),
            Seat::<FourPlayer>::ALL.each_ref()
        );
    }

    #[test]
    fn round_starts_with_zhuangjia_as_actor() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia);

        assert_eq!(round.actor(), &zhuangjia);
    }

    #[test]
    fn round_maps_deal_order_from_zhuangjia_to_fixed_seat_order() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2]);
        let [seat0, seat1, seat2, seat3] = round.players();

        assert_eq!(
            [
                seat0.bingpai().counts()[TileKind::M3.index()],
                seat1.bingpai().counts()[TileKind::M4.index()],
                seat2.bingpai().counts()[TileKind::M1.index()],
                seat3.bingpai().counts()[TileKind::M2.index()],
            ],
            [4, 4, 4, 4]
        );
    }

    #[test]
    fn round_starts_with_seventy_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[0]);

        assert_eq!(round.bipai().remaining_count(), 70);
    }

    #[test]
    fn zimo_consumes_pending_round_and_returns_completed_round() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia);

        let _: Round<FourPlayer, FourPlayerZimoCompleted> = round.zimo().unwrap();
    }
}
