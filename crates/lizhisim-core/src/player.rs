// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::action::DapaiError;
use crate::bingpai::Bingpai;
use crate::he::{He, Sipai};
use crate::player_set::{FourPlayer, PlayerSet};
use crate::seat::Seat;
use crate::tile::TileKind;

pub(crate) enum PlayerDapai {
    Moqie(TileKind),
    ShouqieFromBingpai {
        tile_kind: TileKind,
        zimopai: TileKind,
    },
    ShouqieFromZimopai(TileKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player<P> {
    seat: Seat<P>,
    bingpai: Bingpai,
    he: He,
    first_turn_eligible: bool,
}

impl PlayerSet for FourPlayer {
    const PLAYER_COUNT: usize = 4;
    type Players = [Player<FourPlayer>; Self::PLAYER_COUNT];
    type PointLedger = [crate::table_match::Points; Self::PLAYER_COUNT];
}

impl<P> Player<P> {
    pub(crate) fn from_qipai(seat: Seat<P>, bingpai: Bingpai) -> Self {
        Self {
            seat,
            bingpai,
            he: He::new(),
            first_turn_eligible: true,
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

    pub fn first_turn_eligible(&self) -> bool {
        self.first_turn_eligible
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be called by fulu and babei transitions")
    )]
    pub(crate) fn clear_first_turn_eligibility(mut self) -> Self {
        self.first_turn_eligible = false;
        self
    }

    pub(crate) fn dapai(self, dapai: PlayerDapai) -> Result<Self, DapaiError> {
        let Self {
            seat,
            bingpai,
            he,
            first_turn_eligible: _,
        } = self;

        let (bingpai, sipai) = match dapai {
            PlayerDapai::Moqie(tile_kind) => (
                bingpai,
                Sipai {
                    tile_kind,
                    moqie: true,
                },
            ),
            PlayerDapai::ShouqieFromBingpai { tile_kind, zimopai } => (
                bingpai.with_removed(tile_kind)?.with_added(zimopai)?,
                Sipai {
                    tile_kind,
                    moqie: false,
                },
            ),
            PlayerDapai::ShouqieFromZimopai(tile_kind) => (
                bingpai,
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
            first_turn_eligible: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::bingpai::BingpaiError;
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
            .iter()
            .copied()
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
    fn player_tracks_first_turn_eligibility_independently_of_empty_he() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert_eq!(
            (player.he().iter().next(), player.first_turn_eligible()),
            (None, true)
        );
    }

    #[test]
    fn external_action_can_clear_first_turn_eligibility() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert!(!player.clear_first_turn_eligibility().first_turn_eligible());
    }

    #[test]
    fn shouqie_moves_one_tile_kind_from_bingpai_to_zimopai() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        let player = player
            .dapai(PlayerDapai::ShouqieFromBingpai {
                tile_kind: TileKind::M1,
                zimopai: TileKind::P5,
            })
            .unwrap();

        assert_eq!(
            [
                player.bingpai().counts()[TileKind::M1.index()],
                player.bingpai().counts()[TileKind::P5.index()],
            ],
            [3, 1]
        );
    }

    #[test]
    fn shouqie_appends_non_moqie_sipai() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        let sipai = player
            .dapai(PlayerDapai::ShouqieFromBingpai {
                tile_kind: TileKind::M1,
                zimopai: TileKind::P5,
            })
            .ok()
            .and_then(|player| player.he().last().cloned());

        assert_eq!(
            sipai,
            Some(Sipai {
                tile_kind: TileKind::M1,
                moqie: false,
            })
        );
    }

    #[test]
    fn same_tile_kind_keeps_moqie_and_shouqie_distinct() {
        let mut counts = [0; 37];
        counts[TileKind::P5.index()] = 1;
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            Bingpai::from_validated_counts(counts, TileSet::red_three_four_player()),
        );

        let sipai = [
            player
                .clone()
                .dapai(PlayerDapai::Moqie(TileKind::P5))
                .ok()
                .and_then(|player| player.he().last().cloned()),
            player
                .dapai(PlayerDapai::ShouqieFromBingpai {
                    tile_kind: TileKind::P5,
                    zimopai: TileKind::P5,
                })
                .ok()
                .and_then(|player| player.he().last().cloned()),
        ];

        assert_eq!(
            sipai,
            [
                Some(Sipai {
                    tile_kind: TileKind::P5,
                    moqie: true,
                }),
                Some(Sipai {
                    tile_kind: TileKind::P5,
                    moqie: false,
                }),
            ]
        );
    }

    #[test]
    fn shouqie_from_absent_bingpai_tile_reports_tile_kind() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let player = Player::from_qipai(
            Seat::<FourPlayer>::ALL[0],
            bingpai.into_iter().next().unwrap(),
        );

        assert_eq!(
            player.dapai(PlayerDapai::ShouqieFromBingpai {
                tile_kind: TileKind::Z4,
                zimopai: TileKind::P5,
            }),
            Err(DapaiError::Bingpai(BingpaiError::TileNotPresent {
                tile_kind: TileKind::Z4,
            }))
        );
    }

    #[test]
    fn dapai_with_full_he_returns_he_full_without_player() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();
        let he = (0..27)
            .try_fold(He::new(), |he, _| {
                he.with_appended(Sipai {
                    tile_kind: TileKind::M1,
                    moqie: false,
                })
            })
            .unwrap();
        let player = Player {
            seat: Seat::<FourPlayer>::ALL[0],
            bingpai: bingpai.into_iter().next().unwrap(),
            he,
            first_turn_eligible: true,
        };

        assert_eq!(
            player.dapai(PlayerDapai::Moqie(TileKind::P5)),
            Err(DapaiError::HeFull(crate::he::HeFull))
        );
    }
}
