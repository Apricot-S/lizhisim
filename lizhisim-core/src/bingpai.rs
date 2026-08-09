// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bingpai {
    counts: [u8; 37],
}

impl Default for Bingpai {
    fn default() -> Self {
        Self { counts: [0; 37] }
    }
}

impl Bingpai {
    pub const fn counts(&self) -> &[u8; 37] {
        &self.counts
    }
}

#[cfg(test)]
mod tests {
    use super::Bingpai;

    #[test]
    fn empty_bingpai_has_zero_of_every_tile_kind() {
        assert_eq!(Bingpai::default().counts(), &[0; 37]);
    }
}
