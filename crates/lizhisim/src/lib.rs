// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

pub use lizhisim_core::*;

/// Type-level marker for a four-player game.
pub enum FourPlayer {}

pub mod rules {
    pub use lizhisim_rules::*;
}
