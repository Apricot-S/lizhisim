// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

const MAX_TILE_ID: u8 = 36;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Tile {
    id: u8,
}

#[derive(Debug, Error)]
pub(crate) enum TileError {
    #[error("tile id must be between 0 and 36 but was {0}")]
    OutOfRange(u8),
}

impl Tile {
    /// Creates a new `Tile` from the given ID without performing any range checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `id` is within the valid range of tile identifiers.
    /// Passing an out-of-range value leads to undefined behavior in higher-level logic
    /// that assumes validity.
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked(id: u8) -> Self {
        Self { id }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        self.id
    }

    #[inline(always)]
    #[must_use]
    pub const fn to_usize(&self) -> usize {
        self.id as usize
    }
}

impl TryFrom<u8> for Tile {
    type Error = TileError;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        if id <= MAX_TILE_ID {
            Ok(Self { id })
        } else {
            Err(TileError::OutOfRange(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_valid_7z() {
        assert_eq!(Tile::try_from(33).unwrap(), Tile { id: 33 });
    }

    #[test]
    fn tile_valid_0s() {
        assert_eq!(Tile::try_from(36).unwrap(), Tile { id: 36 });
    }

    #[test]
    fn tile_invalid_id_37() {
        assert!(matches!(
            Tile::try_from(37).unwrap_err(),
            TileError::OutOfRange(37)
        ));
    }
}
