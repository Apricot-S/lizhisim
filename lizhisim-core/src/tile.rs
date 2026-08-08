// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[rustfmt::skip]
pub enum TileKind {
    // wanzi
    M1, M2, M3, M4, M5, M6, M7, M8, M9, M0,
    // tongzi
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P0,
    // suozi
    S1, S2, S3, S4, S5, S6, S7, S8, S9, S0,
    // zipai
    Z1, // dong
    Z2, // nan
    Z3, // xi
    Z4, // bei
    Z5, // bai
    Z6, // fa
    Z7, // zhong
}

impl TileKind {
    #[rustfmt::skip]
    pub const ALL: [Self; 37] = [
        // wanzi
        Self::M1, Self::M2, Self::M3, Self::M4, Self::M5,
        Self::M6, Self::M7, Self::M8, Self::M9, Self::M0,
        // tongzi
        Self::P1, Self::P2, Self::P3, Self::P4, Self::P5,
        Self::P6, Self::P7, Self::P8, Self::P9, Self::P0,
        // suozi
        Self::S1, Self::S2, Self::S3, Self::S4, Self::S5,
        Self::S6, Self::S7, Self::S8, Self::S9, Self::S0,
        // zipai
        Self::Z1, // dong
        Self::Z2, // nan
        Self::Z3, // xi
        Self::Z4, // bei
        Self::Z5, // bai
        Self::Z6, // fa
        Self::Z7, // zhong
    ];
}

#[cfg(test)]
mod tests {
    use super::TileKind;

    #[test]
    fn tile_kind_defines_thirty_seven_kinds() {
        assert_eq!(TileKind::ALL.len(), 37);
    }
}
