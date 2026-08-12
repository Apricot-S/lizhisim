// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bipai::{Bipai, BipaiError, BipaiSpec, QipaiCompleted, QipaiPending};
use crate::player::Player;
use crate::player_set::{FourPlayer, PlayerSet};
use crate::seat::Seat;
use crate::tile::TileKind;

pub struct Round<P: PlayerSet + BipaiSpec, State> {
    bipai: Bipai<P, QipaiCompleted>,
    players: P::Players,
    actor: Seat<P>,
    zhuangjia: Seat<P>,
    state: State,
}

pub struct FourPlayerZimoPending;

pub struct FourPlayerZimoCompleted {
    zimopai: TileKind,
}

impl<P: PlayerSet + BipaiSpec, State> Round<P, State> {
    pub fn bipai(&self) -> &Bipai<P, QipaiCompleted> {
        &self.bipai
    }

    pub fn players(&self) -> &P::Players {
        &self.players
    }

    pub fn actor(&self) -> &Seat<P> {
        &self.actor
    }

    pub fn zhuangjia(&self) -> &Seat<P> {
        &self.zhuangjia
    }
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
            bipai,
            players,
            actor: zhuangjia,
            zhuangjia,
            state: FourPlayerZimoPending,
        }
    }

    pub fn zimo(self) -> Result<Round<FourPlayer, FourPlayerZimoCompleted>, BipaiError> {
        let (bipai, zimopai) = self.bipai.zimo()?;

        Ok(Round {
            bipai,
            players: self.players,
            actor: self.actor,
            zhuangjia: self.zhuangjia,
            state: FourPlayerZimoCompleted { zimopai },
        })
    }
}

impl Round<FourPlayer, FourPlayerZimoCompleted> {
    pub fn zimopai(&self) -> TileKind {
        self.state.zimopai
    }
}

#[cfg(test)]
mod tests {
    use crate::bipai::Bipai;
    use crate::player_set::FourPlayer;
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

    #[test]
    fn zhuangjia_first_zimopai_uses_wall_index_52() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2]);

        let round = round.zimo().unwrap();

        assert_eq!(round.zimopai(), TileKind::P5);
    }

    #[test]
    fn zhuangjia_bingpai_stays_at_thirteen_tiles_after_zimo() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2]);

        let round = round.zimo().unwrap();
        let tile_count: u8 = round.players()[2].bingpai().counts().iter().sum();

        assert_eq!(tile_count, 13);
    }

    #[test]
    fn first_zimo_leaves_sixty_nine_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2]);

        let round = round.zimo().unwrap();

        assert_eq!(round.bipai().remaining_count(), 69);
    }
}
