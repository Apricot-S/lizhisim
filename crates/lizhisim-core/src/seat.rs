// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use core::marker::PhantomData;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FourPlayer;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Seat<P> {
    index: u8,
    player_set: PhantomData<fn() -> P>,
}

#[derive(Debug, Error, PartialEq)]
#[error("seat index {index} is out of range for {seat_count} seats")]
pub struct SeatIndexOutOfRange {
    pub index: usize,
    pub seat_count: usize,
}

impl Seat<FourPlayer> {
    pub const ALL: [Self; 4] = [Self::new(0), Self::new(1), Self::new(2), Self::new(3)];

    const fn new(index: u8) -> Self {
        Self {
            index,
            player_set: PhantomData,
        }
    }
}

impl TryFrom<usize> for Seat<FourPlayer> {
    type Error = SeatIndexOutOfRange;

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        Self::ALL.get(index).copied().ok_or(SeatIndexOutOfRange {
            index,
            seat_count: Self::ALL.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_player_seat_converts_every_valid_index() {
        assert_eq!(
            [0, 1, 2, 3].map(|index| Seat::<FourPlayer>::try_from(index).ok()),
            Seat::<FourPlayer>::ALL.map(Some),
        );
    }

    #[test]
    fn out_of_range_error_reports_index_and_seat_count() {
        assert_eq!(
            Seat::<FourPlayer>::try_from(4),
            Err(SeatIndexOutOfRange {
                index: 4,
                seat_count: 4,
            })
        );
    }
}
