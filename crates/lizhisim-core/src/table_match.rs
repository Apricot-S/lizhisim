// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::player_set::PlayerSet;
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

#[cfg(test)]
mod tests {
    use crate::player_set::FourPlayer;
    use crate::score::Score;

    use super::*;

    fn scores(values: [i32; FourPlayer::PLAYER_COUNT]) -> [Score; FourPlayer::PLAYER_COUNT] {
        values.map(Score::new)
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
}
