// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::player::Player;

mod private {
    pub trait Sealed {}
}

pub trait PlayerSet: private::Sealed {
    type Players;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FourPlayer;

impl private::Sealed for FourPlayer {}

impl PlayerSet for FourPlayer {
    type Players = [Player<FourPlayer>; 4];
}
