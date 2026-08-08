// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FourPlayer;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Seat<P> {
    index: u8,
    player_set: PhantomData<fn() -> P>,
}

impl Seat<FourPlayer> {
    pub const ALL: [Self; 4] = [Self::new(0), Self::new(1), Self::new(2), Self::new(3)];

    pub fn try_from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    const fn new(index: u8) -> Self {
        Self {
            index,
            player_set: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FourPlayer, Seat};

    #[test]
    fn four_player_defines_four_seats() {
        assert_eq!(Seat::<FourPlayer>::ALL.len(), 4);
    }

    #[test]
    fn four_player_seat_converts_every_valid_index() {
        assert_eq!(
            [0, 1, 2, 3].map(Seat::<FourPlayer>::try_from_index),
            Seat::<FourPlayer>::ALL.map(Some)
        );
    }
}
