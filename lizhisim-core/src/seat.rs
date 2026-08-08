// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

#[cfg(test)]
mod tests {
    use super::{FourPlayer, Seat};

    #[test]
    fn four_player_defines_four_seats() {
        assert_eq!(Seat::<FourPlayer>::ALL.len(), 4);
    }
}
