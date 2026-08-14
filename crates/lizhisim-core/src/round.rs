// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::action::{Dapai, DapaiError};
use crate::bipai::{Bipai, BipaiError, BipaiSpec, QipaiCompleted, QipaiPending};
use crate::player::{Player, PlayerDapai};
use crate::player_set::{FourPlayer, PlayerSet};
use crate::seat::Seat;
use crate::tile::TileKind;

pub struct Round<P: PlayerSet + BipaiSpec, State> {
    bipai: Bipai<P, QipaiCompleted>,
    players: P::Players,
    actor: Seat<P>,
    zhuangjia: Seat<P>,
    first_zimo_origin: FirstZimoOrigin,
    state: State,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirstZimoOrigin {
    InitialDeal,
    LiveWall,
}

pub struct ZimoPending;

pub struct ZimoCompleted {
    zimopai: TileKind,
}

pub struct DapaiCompleted;

pub struct RoundEnded {
    outcome: RoundOutcome,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundOutcome {
    HuangpaiPingju,
}

pub enum NoReactionResult<P: PlayerSet + BipaiSpec> {
    NextZimo(Round<P, ZimoPending>),
    RoundEnded(Round<P, RoundEnded>),
}

impl<P: PlayerSet + BipaiSpec> NoReactionResult<P> {
    pub fn into_next_zimo_pending(self) -> Option<Round<P, ZimoPending>> {
        match self {
            Self::NextZimo(round) => Some(round),
            Self::RoundEnded(_) => None,
        }
    }

    pub fn next_zimo_pending(&self) -> Option<&Round<P, ZimoPending>> {
        match self {
            Self::NextZimo(round) => Some(round),
            Self::RoundEnded(_) => None,
        }
    }

    pub fn round_outcome(&self) -> Option<RoundOutcome> {
        match self {
            Self::NextZimo(_) => None,
            Self::RoundEnded(round) => Some(round.round_outcome()),
        }
    }
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

    pub fn first_zimo_origin(&self) -> FirstZimoOrigin {
        self.first_zimo_origin
    }
}

impl Round<FourPlayer, ZimoPending> {
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "will be called by the Round creation boundary")
    )]
    pub(crate) fn new(
        bipai: Bipai<FourPlayer, QipaiPending>,
        zhuangjia: Seat<FourPlayer>,
        first_zimo_origin: FirstZimoOrigin,
    ) -> Self {
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
            first_zimo_origin,
            state: ZimoPending,
        }
    }

    pub fn zimo(self) -> Result<Round<FourPlayer, ZimoCompleted>, BipaiError> {
        let (bipai, zimopai) = self.bipai.zimo()?;

        Ok(Round {
            bipai,
            players: self.players,
            actor: self.actor,
            zhuangjia: self.zhuangjia,
            first_zimo_origin: self.first_zimo_origin,
            state: ZimoCompleted { zimopai },
        })
    }
}

impl Round<FourPlayer, ZimoCompleted> {
    pub fn zimopai(&self) -> TileKind {
        self.state.zimopai
    }

    pub fn dapai(self, dapai: Dapai) -> Result<Round<FourPlayer, DapaiCompleted>, DapaiError> {
        let Self {
            bipai,
            players,
            actor,
            zhuangjia,
            first_zimo_origin,
            state,
        } = self;

        let actor_index = actor.index();
        let player_dapai = match dapai {
            Dapai::Moqie(_)
                if first_zimo_origin == FirstZimoOrigin::InitialDeal
                    && actor == zhuangjia
                    && players[actor_index].first_turn_eligible() =>
            {
                return Err(DapaiError::MoqieUnavailableForInitialDealFirstDapai);
            }
            Dapai::Moqie(tile_kind) => PlayerDapai::Moqie(tile_kind),
            Dapai::Shouqie(tile_kind)
                if first_zimo_origin == FirstZimoOrigin::InitialDeal
                    && actor == zhuangjia
                    && players[actor_index].first_turn_eligible()
                    && tile_kind == state.zimopai =>
            {
                PlayerDapai::ShouqieFromZimopai(tile_kind)
            }
            Dapai::Shouqie(tile_kind) => PlayerDapai::ShouqieFromBingpai {
                tile_kind,
                zimopai: state.zimopai,
            },
        };

        let [player0, player1, player2, player3] = players;
        let players = match actor_index {
            0 => [player0.dapai(player_dapai)?, player1, player2, player3],
            1 => [player0, player1.dapai(player_dapai)?, player2, player3],
            2 => [player0, player1, player2.dapai(player_dapai)?, player3],
            3 => [player0, player1, player2, player3.dapai(player_dapai)?],
            _ => return Err(DapaiError::ActorIndexOutOfRange { actor_index }),
        };

        Ok(Round {
            bipai,
            players,
            actor,
            zhuangjia,
            first_zimo_origin,
            state: DapaiCompleted,
        })
    }
}

impl Round<FourPlayer, DapaiCompleted> {
    pub fn no_reaction(self) -> NoReactionResult<FourPlayer> {
        if self.bipai.remaining_count() == 0 {
            return NoReactionResult::RoundEnded(Round {
                bipai: self.bipai,
                players: self.players,
                actor: self.actor,
                zhuangjia: self.zhuangjia,
                first_zimo_origin: self.first_zimo_origin,
                state: RoundEnded {
                    outcome: RoundOutcome::HuangpaiPingju,
                },
            });
        }

        let actor_index = self.actor.index();
        let next_actor = Seat::<FourPlayer>::ALL[(actor_index + 1) % Seat::<FourPlayer>::ALL.len()];

        NoReactionResult::NextZimo(Round {
            bipai: self.bipai,
            players: self.players,
            actor: next_actor,
            zhuangjia: self.zhuangjia,
            first_zimo_origin: self.first_zimo_origin,
            state: ZimoPending,
        })
    }
}

impl<P: PlayerSet + BipaiSpec> Round<P, RoundEnded> {
    pub fn round_outcome(&self) -> RoundOutcome {
        self.state.outcome
    }
}

#[cfg(test)]
mod tests {
    use crate::bipai::Bipai;
    use crate::he::Sipai;
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
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[0],
            FirstZimoOrigin::InitialDeal,
        );

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
        let round = Round::new(bipai, zhuangjia, FirstZimoOrigin::InitialDeal);

        assert_eq!(round.actor(), &zhuangjia);
    }

    #[test]
    fn round_maps_deal_order_from_zhuangjia_to_fixed_seat_order() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        );
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
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[0],
            FirstZimoOrigin::InitialDeal,
        );

        assert_eq!(round.bipai().remaining_count(), 70);
    }

    #[test]
    fn zimo_consumes_pending_round_and_returns_completed_round() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia, FirstZimoOrigin::InitialDeal);

        let _: Round<FourPlayer, ZimoCompleted> = round.zimo().unwrap();
    }

    #[test]
    fn zhuangjia_first_zimopai_uses_wall_index_52() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        );

        let round = round.zimo().unwrap();

        assert_eq!(round.zimopai(), TileKind::P5);
    }

    #[test]
    fn zhuangjia_bingpai_stays_at_thirteen_tiles_after_zimo() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        );

        let round = round.zimo().unwrap();
        let tile_count: u8 = round.players()[2].bingpai().counts().iter().sum();

        assert_eq!(tile_count, 13);
    }

    #[test]
    fn first_zimo_leaves_sixty_nine_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        );

        let round = round.zimo().unwrap();

        assert_eq!(round.bipai().remaining_count(), 69);
    }

    #[test]
    fn first_zimo_preserves_configured_origin() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        );

        let round = round.zimo().unwrap();

        assert_eq!(round.first_zimo_origin(), FirstZimoOrigin::InitialDeal);
    }

    #[test]
    fn initial_deal_zimopai_can_be_shouqie() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        )
        .zimo()
        .unwrap();
        let bingpai_counts = *round.players()[2].bingpai().counts();

        let result = round.dapai(Dapai::Shouqie(TileKind::P5)).ok().map(|round| {
            (
                *round.players()[2].bingpai().counts(),
                round.players()[2].he().last().cloned(),
            )
        });

        assert_eq!(
            result,
            Some((
                bingpai_counts,
                Some(Sipai {
                    tile_kind: TileKind::P5,
                    moqie: false,
                }),
            ))
        );
    }

    #[test]
    fn initial_deal_first_dapai_rejects_moqie() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        )
        .zimo()
        .unwrap();

        assert_eq!(
            round.dapai(Dapai::Moqie(TileKind::P5)).map(|_| ()),
            Err(DapaiError::MoqieUnavailableForInitialDealFirstDapai)
        );
    }

    #[test]
    fn initial_deal_zimopai_shouqie_exception_requires_zhuangjia_actor() {
        let (mut tiles, tile_set) = red_three_tiles();
        tiles.swap(53, 120);
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let transition = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        )
        .zimo()
        .unwrap()
        .dapai(Dapai::Shouqie(TileKind::P5))
        .unwrap()
        .no_reaction();
        let result = transition
            .into_next_zimo_pending()
            .unwrap()
            .zimo()
            .unwrap()
            .dapai(Dapai::Shouqie(TileKind::Z4))
            .map(|_| ());

        assert_eq!(
            result,
            Err(DapaiError::Bingpai(
                crate::bingpai::BingpaiError::TileNotPresent {
                    tile_kind: TileKind::Z4,
                }
            ))
        );
    }

    #[test]
    fn live_wall_zimopai_dapai_is_moqie() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2], FirstZimoOrigin::LiveWall)
            .zimo()
            .unwrap();

        let sipai = round
            .dapai(Dapai::Moqie(TileKind::P5))
            .ok()
            .and_then(|round| round.players()[2].he().last().cloned());

        assert_eq!(
            sipai,
            Some(Sipai {
                tile_kind: TileKind::P5,
                moqie: true,
            })
        );
    }

    #[test]
    fn moqie_preserves_actor_bingpai_counts() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2], FirstZimoOrigin::LiveWall)
            .zimo()
            .unwrap();
        let bingpai_counts = *round.players()[2].bingpai().counts();

        let result = round
            .dapai(Dapai::Moqie(TileKind::P5))
            .map(|round| *round.players()[2].bingpai().counts());

        assert_eq!(result, Ok(bingpai_counts));
    }

    #[test]
    fn moqie_appends_zimopai_tile_kind_to_actor_he() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2], FirstZimoOrigin::LiveWall)
            .zimo()
            .unwrap();

        let result = round
            .dapai(Dapai::Moqie(TileKind::P5))
            .ok()
            .and_then(|round| round.players()[2].he().last().map(|sipai| sipai.tile_kind));

        assert_eq!(result, Some(TileKind::P5));
    }

    #[test]
    fn dapai_completed_actor_is_dapai_actor() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let zhuangjia = Seat::<FourPlayer>::ALL[2];
        let round = Round::new(bipai, zhuangjia, FirstZimoOrigin::LiveWall)
            .zimo()
            .unwrap();

        let result = round
            .dapai(Dapai::Moqie(TileKind::P5))
            .map(|round| *round.actor());

        assert_eq!(result, Ok(zhuangjia));
    }

    #[test]
    fn first_dapai_clears_actor_first_turn_eligibility() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::InitialDeal,
        )
        .zimo()
        .unwrap();

        let first_dapai_round = round.dapai(Dapai::Shouqie(TileKind::P5)).unwrap();

        assert_eq!(
            first_dapai_round
                .players()
                .each_ref()
                .map(Player::first_turn_eligible),
            [true, true, false, true]
        );
    }

    #[test]
    fn no_reaction_advances_every_seat_in_fixed_order() {
        let next_actors = Seat::<FourPlayer>::ALL.map(|zhuangjia| {
            let (tiles, tile_set) = red_three_tiles();
            let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();

            Round::new(bipai, zhuangjia, FirstZimoOrigin::LiveWall)
                .zimo()
                .unwrap()
                .dapai(Dapai::Moqie(TileKind::P5))
                .unwrap()
                .no_reaction()
                .next_zimo_pending()
                .map(|round| *round.actor())
        });

        assert_eq!(
            next_actors,
            [
                Some(Seat::<FourPlayer>::ALL[1]),
                Some(Seat::<FourPlayer>::ALL[2]),
                Some(Seat::<FourPlayer>::ALL[3]),
                Some(Seat::<FourPlayer>::ALL[0]),
            ]
        );
    }

    #[test]
    fn no_reaction_preserves_bipai_and_players() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let round = Round::new(bipai, Seat::<FourPlayer>::ALL[2], FirstZimoOrigin::LiveWall)
            .zimo()
            .unwrap()
            .dapai(Dapai::Moqie(TileKind::P5))
            .unwrap();
        let before = (round.bipai().clone(), round.players().clone());
        let after = match round.no_reaction() {
            NoReactionResult::NextZimo(round) => {
                Some((round.bipai().clone(), round.players().clone()))
            }
            NoReactionResult::RoundEnded(_) => None,
        };

        assert_eq!(after, Some(before));
    }

    #[test]
    fn no_reaction_then_zimo_gives_next_wall_tile_to_next_actor() {
        let (mut tiles, tile_set) = red_three_tiles();
        tiles.swap(53, 120);
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let result = Round::new(bipai, Seat::<FourPlayer>::ALL[2], FirstZimoOrigin::LiveWall)
            .zimo()
            .unwrap()
            .dapai(Dapai::Moqie(TileKind::P5))
            .unwrap()
            .no_reaction();
        let actor_and_zimopai = match result {
            NoReactionResult::NextZimo(round) => round
                .zimo()
                .ok()
                .map(|round| (*round.actor(), round.zimopai())),
            NoReactionResult::RoundEnded(_) => None,
        };

        assert_eq!(
            actor_and_zimopai,
            Some((Seat::<FourPlayer>::ALL[3], TileKind::Z4))
        );
    }

    #[test]
    fn no_reaction_after_all_live_wall_tiles_returns_huangpai_pingju() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let mut transition = NoReactionResult::NextZimo(Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::LiveWall,
        ));

        for _ in 0..70 {
            transition = match transition {
                NoReactionResult::NextZimo(round) => round
                    .zimo()
                    .unwrap()
                    .dapai(Dapai::Moqie(TileKind::P5))
                    .unwrap()
                    .no_reaction(),
                NoReactionResult::RoundEnded(_) => break,
            };
        }

        assert_eq!(
            transition.round_outcome(),
            Some(RoundOutcome::HuangpaiPingju)
        );
    }

    #[test]
    fn no_reaction_with_one_live_wall_tile_returns_next_zimo() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let mut transition = NoReactionResult::NextZimo(Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::LiveWall,
        ));

        for _ in 0..69 {
            transition = match transition {
                NoReactionResult::NextZimo(round) => round
                    .zimo()
                    .unwrap()
                    .dapai(Dapai::Moqie(TileKind::P5))
                    .unwrap()
                    .no_reaction(),
                NoReactionResult::RoundEnded(_) => break,
            };
        }

        assert_eq!(
            transition
                .next_zimo_pending()
                .map(|round| round.bipai().remaining_count()),
            Some(1)
        );
    }

    #[test]
    fn huangpai_pingju_moves_bipai_and_players_to_round_ended() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let mut transition = NoReactionResult::NextZimo(Round::new(
            bipai,
            Seat::<FourPlayer>::ALL[2],
            FirstZimoOrigin::LiveWall,
        ));

        for _ in 0..69 {
            transition = match transition {
                NoReactionResult::NextZimo(round) => round
                    .zimo()
                    .unwrap()
                    .dapai(Dapai::Moqie(TileKind::P5))
                    .unwrap()
                    .no_reaction(),
                NoReactionResult::RoundEnded(_) => break,
            };
        }

        let round = match transition {
            NoReactionResult::NextZimo(round) => round
                .zimo()
                .unwrap()
                .dapai(Dapai::Moqie(TileKind::P5))
                .unwrap(),
            NoReactionResult::RoundEnded(_) => unreachable!(),
        };
        let before = (round.bipai().clone(), round.players().clone());
        let after = match round.no_reaction() {
            NoReactionResult::NextZimo(_) => None,
            NoReactionResult::RoundEnded(round) => {
                Some((round.bipai().clone(), round.players().clone()))
            }
        };

        assert_eq!(after, Some(before));
    }
}
