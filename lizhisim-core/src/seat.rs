// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

#[cfg(test)]
mod tests {
    use super::{FourPlayer, Seat};

    #[test]
    fn four_player_seat_accepts_exactly_four_indices() {
        assert_eq!(Seat::<FourPlayer>::ALL.len(), 4);

        for (index, expected) in Seat::<FourPlayer>::ALL.iter().enumerate() {
            assert_eq!(
                Seat::<FourPlayer>::try_from_index(index).as_ref(),
                Ok(expected)
            );
        }

        assert!(Seat::<FourPlayer>::try_from_index(4).is_err());
        assert!(Seat::<FourPlayer>::try_from_index(usize::MAX).is_err());
    }
}
