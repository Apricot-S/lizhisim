// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum TileKind {
    // wanzi
    M1, M2, M3, M4, M5, M6, M7, M8, M9,
    // tongzi
    P1, P2, P3, P4, P5, P6, P7, P8, P9,
    // suozi
    S1, S2, S3, S4, S5, S6, S7, S8, S9,
    // zipai
    Z1, // dong
    Z2, // nan
    Z3, // xi
    Z4, // bei
    Z5, // bai
    Z6, // fa
    Z7, // zhong
    // hong_baopai
    M0, P0, S0,
}

impl TileKind {
    pub const fn index(self) -> usize {
        self as usize
    }

    #[rustfmt::skip]
    pub const ALL: [Self; 37] = [
        // wanzi
        Self::M1, Self::M2, Self::M3, Self::M4, Self::M5,
        Self::M6, Self::M7, Self::M8, Self::M9,
        // tongzi
        Self::P1, Self::P2, Self::P3, Self::P4, Self::P5,
        Self::P6, Self::P7, Self::P8, Self::P9,
        // suozi
        Self::S1, Self::S2, Self::S3, Self::S4, Self::S5,
        Self::S6, Self::S7, Self::S8, Self::S9,
        // zipai
        Self::Z1, // dong
        Self::Z2, // nan
        Self::Z3, // xi
        Self::Z4, // bei
        Self::Z5, // bai
        Self::Z6, // fa
        Self::Z7, // zhong
        // hong_baopai
        Self::M0, Self::P0, Self::S0,
    ];

    pub const fn is_hong_baopai(self) -> bool {
        matches!(self, Self::M0 | Self::P0 | Self::S0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_kind_defines_thirty_seven_kinds() {
        assert_eq!(TileKind::ALL.len(), 37);
    }

    #[test]
    #[rustfmt::skip]
    fn tile_kind_orders_base_kinds_before_red_kinds() {
        assert_eq!(TileKind::ALL, [
            TileKind::M1, TileKind::M2, TileKind::M3, TileKind::M4, TileKind::M5,
            TileKind::M6, TileKind::M7, TileKind::M8, TileKind::M9,
            TileKind::P1, TileKind::P2, TileKind::P3, TileKind::P4, TileKind::P5,
            TileKind::P6, TileKind::P7, TileKind::P8, TileKind::P9,
            TileKind::S1, TileKind::S2, TileKind::S3, TileKind::S4, TileKind::S5,
            TileKind::S6, TileKind::S7, TileKind::S8, TileKind::S9,
            TileKind::Z1, TileKind::Z2, TileKind::Z3, TileKind::Z4,
            TileKind::Z5, TileKind::Z6, TileKind::Z7,
            TileKind::M0, TileKind::P0, TileKind::S0,
        ]);
    }

    #[test]
    fn zero_numbered_suit_kinds_are_hong_baopai() {
        assert_eq!(
            [TileKind::M0, TileKind::P0, TileKind::S0].map(TileKind::is_hong_baopai),
            [true; 3],
        );
    }

    #[test]
    fn base_kinds_are_not_hong_baopai() {
        assert!(
            TileKind::ALL[..34]
                .iter()
                .copied()
                .all(|tile_kind| !tile_kind.is_hong_baopai())
        );
    }
}
