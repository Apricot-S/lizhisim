// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::action::{Dapai, DapaiError};
use crate::bingpai::Bingpai;
use crate::he::{He, Sipai};
use crate::player_set::{FourPlayer, PlayerSet};
use crate::seat::Seat;
use crate::tile::TileKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player<P> {
    seat: Seat<P>,
    bingpai: Bingpai,
    he: He,
    first_dapai_pending: bool,
}

impl<P> Player<P> {
    pub(crate) fn from_qipai(seat: Seat<P>, bingpai: Bingpai) -> Self {
        Self {
            seat,
            bingpai,
            he: He::new(),
            first_dapai_pending: true,
        }
    }

    pub fn seat(&self) -> &Seat<P> {
        &self.seat
    }

    pub fn bingpai(&self) -> &Bingpai {
        &self.bingpai
    }

    pub fn he(&self) -> &He {
        &self.he
    }

    pub fn first_dapai_pending(&self) -> bool {
        self.first_dapai_pending
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be called by fulu and babei transitions")
    )]
    pub(crate) fn clear_first_dapai_pending(mut self) -> Self {
        self.first_dapai_pending = false;
        self
    }

    pub(crate) fn dapai(
        self,
        dapai: Dapai,
        zimopai: TileKind,
        moqie: bool,
    ) -> Result<Self, DapaiError> {
        let Self {
            seat,
            bingpai,
            he,
            first_dapai_pending: _,
        } = self;

        let (bingpai, sipai) = match dapai {
            Dapai::Moqie(tile_kind) => (bingpai, Sipai { tile_kind, moqie }),
            Dapai::Shouqie(tile_kind) => (
                bingpai.with_removed(tile_kind)?.with_added(zimopai)?,
                Sipai {
                    tile_kind,
                    moqie: false,
                },
            ),
        };
        let he = he.with_appended(sipai)?;

        Ok(Self {
            seat,
            bingpai,
            he,
            first_dapai_pending: false,
        })
    }
}

impl PlayerSet for FourPlayer {
    type Players = [Player<FourPlayer>; 4];
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
    fn player_preserves_qipai_bingpai_counts() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let expected = *bingpai[0].counts();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert_eq!(*player.bingpai().counts(), expected);
    }

    #[test]
    fn player_preserves_seat_from_qipai() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let players = Seat::<FourPlayer>::ALL
            .into_iter()
            .zip(bingpai)
            .map(|(seat, bingpai)| Player::from_qipai(seat, bingpai))
            .collect::<Vec<_>>();

        assert_eq!(
            players.iter().map(Player::seat).collect::<Vec<_>>(),
            Seat::<FourPlayer>::ALL.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn player_has_empty_he_after_qipai() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert_eq!(player.he().iter().next(), None);
    }

    #[test]
    fn player_tracks_first_dapai_pending_independently_of_empty_he() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert_eq!(
            (player.he().iter().next(), player.first_dapai_pending()),
            (None, true)
        );
    }

    #[test]
    fn external_action_can_clear_first_dapai_pending() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert!(!player.clear_first_dapai_pending().first_dapai_pending());
    }
}
