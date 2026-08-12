// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bipai::BipaiSpec;
use crate::player::Player;
use crate::seat::FourPlayer;

mod private {
    pub trait Sealed {}
}

pub trait PlayerSet: BipaiSpec + private::Sealed {
    type Players;
}

impl private::Sealed for FourPlayer {}

impl PlayerSet for FourPlayer {
    type Players = [Player<FourPlayer>; 4];
}
