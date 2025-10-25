// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

/// Converts a tile literal into a `u8` tile ID.
///
/// This macro maps tile notations such as `1m`, `9p`, `5s`, `7z`, or `0s` (red five)
/// into their corresponding tile IDs.
/// The mapping is resolved entirely at compile time, and invalid literals
/// will cause a compilation error.
///
/// # Safety
///
/// This macro is safe to use. This macro itself guarantees that only valid literals are accepted,
/// so the generated `u8` is always within the expected range.
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(tu8!(1m), 0u8);  // 一萬
/// assert_eq!(tu8!(2p), 10u8); // 二筒
/// assert_eq!(tu8!(1z), 27u8); // 東
/// assert_eq!(tu8!(0s), 36u8); // 赤五索
/// ```
#[macro_export]
macro_rules! tu8 {
    (1m) => {
        0u8
    };
    (2m) => {
        1u8
    };
    (3m) => {
        2u8
    };
    (4m) => {
        3u8
    };
    (5m) => {
        4u8
    };
    (6m) => {
        5u8
    };
    (7m) => {
        6u8
    };
    (8m) => {
        7u8
    };
    (9m) => {
        8u8
    };

    (1p) => {
        9u8
    };
    (2p) => {
        10u8
    };
    (3p) => {
        11u8
    };
    (4p) => {
        12u8
    };
    (5p) => {
        13u8
    };
    (6p) => {
        14u8
    };
    (7p) => {
        15u8
    };
    (8p) => {
        16u8
    };
    (9p) => {
        17u8
    };

    (1s) => {
        18u8
    };
    (2s) => {
        19u8
    };
    (3s) => {
        20u8
    };
    (4s) => {
        21u8
    };
    (5s) => {
        22u8
    };
    (6s) => {
        23u8
    };
    (7s) => {
        24u8
    };
    (8s) => {
        25u8
    };
    (9s) => {
        26u8
    };

    (1z) => {
        27u8
    };
    (2z) => {
        28u8
    };
    (3z) => {
        29u8
    };
    (4z) => {
        30u8
    };
    (5z) => {
        31u8
    };
    (6z) => {
        32u8
    };
    (7z) => {
        33u8
    };

    (0m) => {
        34u8
    };
    (0p) => {
        35u8
    };
    (0s) => {
        36u8
    };

    () => {
        ::std::compile_error!("No tile name provided. Specify a tile name.");
    };

    ($invalid:tt) => {
        ::std::compile_error!(::std::concat!(
            "Invalid tile name: ",
            ::std::stringify!($invalid)
        ));
    };
}

/// Converts a tile literal into a `usize` tile ID.
///
/// This macro behaves the same as [`tu8!`], except that it produces a `usize` instead of a `u8`.
///
/// For details on the mapping and supported literals, see the documentation of [`tu8!`].
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(tuz!(1m), 0usize);  // 一萬
/// assert_eq!(tuz!(2p), 10usize); // 二筒
/// assert_eq!(tuz!(1z), 27usize); // 東
/// assert_eq!(tuz!(0s), 36usize); // 赤五索
/// ```
#[macro_export]
macro_rules! tuz {
    ($name:tt) => {
        $crate::tu8!($name) as usize
    };
}

/// Creates a [`Tile`](crate::tile::Tile) from a tile name literal.
///
/// For details on the mapping and supported literals, see the documentation of [`tu8!`].
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(t!(1m), Tile::try_from(0).unwrap());  // 一萬
/// assert_eq!(t!(2p), Tile::try_from(10).unwrap()); // 二筒
/// assert_eq!(t!(1z), Tile::try_from(27).unwrap()); // 東
/// assert_eq!(t!(0s), Tile::try_from(36).unwrap()); // 赤五索
/// ```
#[macro_export]
macro_rules! t {
    ($name:tt) => {
        // SAFETY: `tu8!` guarantees that `$name` is a valid tile literal.
        unsafe { $crate::tile::Tile::new_unchecked($crate::tu8!($name)) }
    };
}

#[macro_export]
macro_rules! matches_tu8 {
    ($index:expr, $($name:tt)|+) => {
        ::std::matches!($index, $($crate::tu8!($name))|+)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::Tile;

    #[test]
    fn tu8_1m() {
        assert_eq!(tu8!(1m), 0u8);
    }

    #[test]
    fn tu8_1z() {
        assert_eq!(tu8!(1z), 27u8);
    }

    #[test]
    fn tu8_0s() {
        assert_eq!(tu8!(0s), 36u8);
    }

    #[test]
    fn tuz_1m() {
        assert_eq!(tuz!(1m), 0usize);
    }

    #[test]
    fn t_1m() {
        assert_eq!(t!(1m), Tile::try_from(0).unwrap());
    }
}
