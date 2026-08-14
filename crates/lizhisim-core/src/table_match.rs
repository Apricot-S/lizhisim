// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bipai::BipaiSpec;
use crate::player_set::PlayerSet;
use crate::round::{Round, RoundEnded, RoundOutcome};
use crate::seat::Seat;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Chang {
    Dong,
    Nan,
    Xi,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RoundIndex(u16);

impl RoundIndex {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ben(u16);

impl Ben {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Lizhibang(u16);

impl Lizhibang {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TableMatchState<P: PlayerSet> {
    chang: Chang,
    round_index: RoundIndex,
    ben: Ben,
    lizhibang: Lizhibang,
    zhuangjia: Seat<P>,
    scores: P::Scores,
}

pub struct RoundSettlement<P: PlayerSet + BipaiSpec> {
    table_match_state: TableMatchState<P>,
    round: Round<P, RoundEnded>,
}

impl<P: PlayerSet> TableMatchState<P> {
    pub fn new(
        chang: Chang,
        round_index: RoundIndex,
        ben: Ben,
        lizhibang: Lizhibang,
        zhuangjia: Seat<P>,
        scores: P::Scores,
    ) -> Self {
        Self {
            chang,
            round_index,
            ben,
            lizhibang,
            zhuangjia,
            scores,
        }
    }

    pub fn chang(&self) -> Chang {
        self.chang
    }

    pub fn round_index(&self) -> RoundIndex {
        self.round_index
    }

    pub fn ben(&self) -> Ben {
        self.ben
    }

    pub fn lizhibang(&self) -> Lizhibang {
        self.lizhibang
    }

    pub fn zhuangjia(&self) -> Seat<P> {
        self.zhuangjia
    }

    pub fn scores(&self) -> &P::Scores {
        &self.scores
    }
}

impl<P: PlayerSet + BipaiSpec> TableMatchState<P> {
    pub fn into_round_settlement(self, round: Round<P, RoundEnded>) -> RoundSettlement<P> {
        RoundSettlement {
            table_match_state: self,
            round,
        }
    }
}

impl<P: PlayerSet + BipaiSpec> RoundSettlement<P> {
    pub fn table_match_state(&self) -> &TableMatchState<P> {
        &self.table_match_state
    }

    pub fn round_outcome(&self) -> RoundOutcome {
        self.round.round_outcome()
    }
}

#[cfg(test)]
mod tests {
    use crate::action::Dapai;
    use crate::bipai::Bipai;
    use crate::player_set::FourPlayer;
    use crate::round::{FirstZimoOrigin, NoReactionResult, Round, RoundEnded, RoundOutcome};
    use crate::score::Score;
    use crate::tile::TileKind;
    use crate::tile_set::TileSet;

    use super::*;

    fn scores(values: [i32; FourPlayer::PLAYER_COUNT]) -> [Score; FourPlayer::PLAYER_COUNT] {
        values.map(Score::new)
    }

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

    fn huangpai_pingju_round() -> Round<FourPlayer, RoundEnded> {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let mut transition =
            NoReactionResult::NextZimo(Round::new(bipai, Seat::ALL[0], FirstZimoOrigin::LiveWall));

        for _ in 0..70 {
            transition = match transition {
                NoReactionResult::NextZimo(round) => {
                    let round = round.zimo().unwrap();
                    let dapai = Dapai::Moqie(round.zimopai());
                    round.dapai(dapai).unwrap().no_reaction()
                }
                NoReactionResult::RoundEnded(_) => break,
            };
        }

        match transition {
            NoReactionResult::NextZimo(_) => unreachable!(),
            NoReactionResult::RoundEnded(round) => round,
        }
    }

    #[test]
    fn table_match_state_keeps_one_score_for_each_four_player_seat() {
        let state = TableMatchState::new(
            Chang::Dong,
            RoundIndex::new(0),
            Ben::new(0),
            Lizhibang::new(0),
            Seat::ALL[0],
            scores([25_000, 25_000, 25_000, 25_000]),
        );

        assert_eq!(
            (
                state.scores().len(),
                state.scores().map(|score| score.value()),
                state.chang(),
                state.zhuangjia(),
                state.ben(),
                state.lizhibang(),
            ),
            (
                Seat::<FourPlayer>::ALL.len(),
                [25_000; FourPlayer::PLAYER_COUNT],
                Chang::Dong,
                Seat::<FourPlayer>::ALL[0],
                Ben::new(0),
                Lizhibang::new(0),
            )
        );
    }

    #[test]
    fn round_ended_is_consumed_to_create_round_settlement() {
        let state = TableMatchState::new(
            Chang::Dong,
            RoundIndex::new(0),
            Ben::new(0),
            Lizhibang::new(0),
            Seat::ALL[0],
            scores([25_000; FourPlayer::PLAYER_COUNT]),
        );
        let settlement = state.into_round_settlement(huangpai_pingju_round());

        assert_eq!(settlement.round_outcome(), RoundOutcome::HuangpaiPingju);
    }
}
