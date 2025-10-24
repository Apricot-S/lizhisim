// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

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

#[macro_export]
macro_rules! tuz {
    ($name:tt) => {
        $crate::tu8!($name) as usize
    };
}

#[macro_export]
macro_rules! matches_tu8 {
    ($index:expr, $($name:tt)|+) => {
        ::std::matches!($index, $($crate::tu8!($name))|+)
    };
}
