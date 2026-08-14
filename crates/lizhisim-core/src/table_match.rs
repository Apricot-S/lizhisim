// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::player_set::{FourPlayer, PlayerSet};
use crate::seat::Seat;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Points(i32);

impl Points {
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> i32 {
        self.0
    }
}

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
    points: [Points; FourPlayer::PLAYER_COUNT],
    chang: Chang,
    round_index: RoundIndex,
    zhuangjia: Seat<P>,
    ben: Ben,
    lizhibang: Lizhibang,
}

impl TableMatchState<FourPlayer> {
    pub fn new(
        points: [Points; FourPlayer::PLAYER_COUNT],
        chang: Chang,
        round_index: RoundIndex,
        zhuangjia: Seat<FourPlayer>,
        ben: Ben,
        lizhibang: Lizhibang,
    ) -> Self {
        Self {
            points,
            chang,
            round_index,
            zhuangjia,
            ben,
            lizhibang,
        }
    }

    pub fn points(&self) -> &[Points; FourPlayer::PLAYER_COUNT] {
        &self.points
    }

    pub fn chang(&self) -> Chang {
        self.chang
    }

    pub fn round_index(&self) -> RoundIndex {
        self.round_index
    }

    pub fn zhuangjia(&self) -> Seat<FourPlayer> {
        self.zhuangjia
    }

    pub fn ben(&self) -> Ben {
        self.ben
    }

    pub fn lizhibang(&self) -> Lizhibang {
        self.lizhibang
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn points(values: [i32; FourPlayer::PLAYER_COUNT]) -> [Points; FourPlayer::PLAYER_COUNT] {
        values.map(Points::new)
    }

    #[test]
    fn table_match_state_keeps_one_point_entry_for_each_four_player_seat() {
        let state = TableMatchState::new(
            points([25_000, 25_000, 25_000, 25_000]),
            Chang::Dong,
            RoundIndex::new(0),
            Seat::ALL[0],
            Ben::new(0),
            Lizhibang::new(0),
        );

        assert_eq!(
            (
                state.points().len(),
                state.points().map(|point| point.value()),
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
