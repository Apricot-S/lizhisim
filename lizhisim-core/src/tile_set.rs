// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TileSetError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileSet {
    counts: [u8; 37],
}

impl TileSet {
    pub const fn try_from_counts(counts: [u8; 37]) -> Result<Self, TileSetError> {
        Ok(Self { counts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physically_possible_counts_construct_tile_set() {
        assert!(TileSet::try_from_counts([0; 37]).is_ok());
    }
}
