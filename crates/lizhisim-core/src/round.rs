// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bipai::{Bipai, BipaiError, BipaiSpec, QipaiCompleted, QipaiPending};
use crate::player::Player;
use crate::seat::FourPlayer;
use crate::seat::Seat;
use crate::tile::TileKind;

pub struct Round<P, State> {
    state: State,
    zhuangjia: Seat<P>,
}

pub struct RoundQipaiPending<P: BipaiSpec> {
    bipai: Bipai<P, QipaiPending>,
}

pub struct FourPlayerZimoPending {
    bipai: Bipai<FourPlayer, QipaiCompleted>,
    players: [Player<FourPlayer>; 4],
    actor: Seat<FourPlayer>,
}

pub struct FourPlayerZimoCompleted {
    pending: FourPlayerZimoPending,
    zimopai: TileKind,
}

impl<P: BipaiSpec> Round<P, RoundQipaiPending<P>> {
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be called by the Round creation boundary")
    )]
    pub(crate) fn new(bipai: Bipai<P, QipaiPending>, zhuangjia: Seat<P>) -> Self {
        Self {
            state: RoundQipaiPending { bipai },
            zhuangjia,
        }
    }

    pub fn bipai(&self) -> &Bipai<P, QipaiPending> {
        &self.state.bipai
    }
}

impl Round<FourPlayer, RoundQipaiPending<FourPlayer>> {
    pub fn qipai(self) -> Round<FourPlayer, FourPlayerZimoPending> {
        let (bipai, mut bingpai) = self.state.bipai.qipai();
        bingpai.rotate_right(self.zhuangjia.index());
        let [bingpai0, bingpai1, bingpai2, bingpai3] = bingpai;
        let [seat0, seat1, seat2, seat3] = Seat::<FourPlayer>::ALL;
        let players = [
            Player::from_qipai(seat0, bingpai0),
            Player::from_qipai(seat1, bingpai1),
            Player::from_qipai(seat2, bingpai2),
            Player::from_qipai(seat3, bingpai3),
        ];

        Round {
            state: FourPlayerZimoPending {
                bipai,
                players,
                actor: self.zhuangjia,
            },
            zhuangjia: self.zhuangjia,
        }
    }
}

impl Round<FourPlayer, FourPlayerZimoPending> {
    pub fn bipai(&self) -> &Bipai<FourPlayer, QipaiCompleted> {
        &self.state.bipai
    }

    pub fn players(&self) -> &[Player<FourPlayer>; 4] {
        &self.state.players
    }

    pub fn actor(&self) -> &Seat<FourPlayer> {
        &self.state.actor
    }

    pub fn zimo(self) -> Result<Round<FourPlayer, FourPlayerZimoCompleted>, BipaiError> {
        let (bipai, zimopai) = self.state.bipai.zimo()?;
        let pending = FourPlayerZimoPending {
            bipai,
            players: self.state.players,
            actor: self.state.actor,
        };

        Ok(Round {
            state: FourPlayerZimoCompleted { pending, zimopai },
            zhuangjia: self.zhuangjia,
        })
    }
}

impl Round<FourPlayer, FourPlayerZimoCompleted> {
    pub fn actor(&self) -> &Seat<FourPlayer> {
        &self.state.pending.actor
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

        let round: Round<FourPlayer, FourPlayerZimoPending> = round.qipai();

        assert_eq!(round.zhuangjia(), &zhuangjia);
    }

    #[test]
    fn qipai_round_preserves_four_players() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[0]).qipai();

        assert_eq!(
            round.players().each_ref().map(Player::seat),
            Seat::<FourPlayer>::ALL.each_ref()
        );
    }

    #[test]
    fn qipai_round_starts_with_zhuangjia_as_actor() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia).qipai();

        assert_eq!(round.actor(), &zhuangjia);
    }

    #[test]
    fn qipai_maps_deal_order_from_zhuangjia_to_fixed_seat_order() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2]).qipai();
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
    fn qipai_round_has_seventy_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[0]).qipai();

        assert_eq!(round.bipai().remaining_count(), 70);
    }

    #[test]
    fn zimo_consumes_pending_round_and_returns_completed_round() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia).qipai();

        let _: Round<FourPlayer, FourPlayerZimoCompleted> = round.zimo().unwrap();
    }
}
