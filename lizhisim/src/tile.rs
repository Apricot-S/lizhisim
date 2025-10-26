// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::{matches_tu8, t, tu8};
use thiserror::Error;

const MAX_TILE_ID: u8 = 36;

/// 牌: Tile.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    #[inline]
    #[must_use]
    pub(crate) const fn as_u8(&self) -> u8 {
        self.id
    }

    #[inline]
    #[must_use]
    pub(crate) const fn to_usize(self) -> usize {
        self.id as usize
    }

    #[allow(clippy::manual_range_patterns)]
    #[inline]
    #[must_use]
    pub(crate) const fn is_zipai(&self) -> bool {
        matches_tu8!(self.id, 1z | 2z | 3z | 4z | 5z | 6z | 7z)
    }

    #[inline]
    #[must_use]
    pub(crate) const fn is_yaojiupai(&self) -> bool {
        matches_tu8!(
            self.id,
            1m | 9m | 1p | 9p | 1s | 9s | 1z | 2z | 3z | 4z | 5z | 6z | 7z
        )
    }

    #[allow(clippy::manual_range_patterns)]
    #[inline]
    #[must_use]
    pub(crate) const fn is_hongbaopai(&self) -> bool {
        matches_tu8!(self.id, 0m | 0p | 0s)
    }

    #[inline]
    #[must_use]
    pub(crate) const fn normalize_hongbaopai(&self) -> Self {
        match self.id {
            tu8!(0m) => t!(5m),
            tu8!(0p) => t!(5p),
            tu8!(0s) => t!(5s),
            _ => *self,
        }
    }

    #[inline]
    #[must_use]
    pub(crate) const fn decorate_hongbaopai(&self) -> Self {
        match self.id {
            tu8!(5m) => t!(0m),
            tu8!(5p) => t!(0p),
            tu8!(5s) => t!(0s),
            _ => *self,
        }
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

    #[test]
    fn is_zipai_7z() {
        let t = Tile::try_from(33).unwrap();
        assert!(t.is_zipai());
    }

    #[test]
    fn is_not_zipai_0m() {
        let t = Tile::try_from(34).unwrap();
        assert!(!t.is_zipai());
    }

    #[test]
    fn is_yaojiupai_1m() {
        let t = Tile::try_from(0).unwrap();
        assert!(t.is_yaojiupai());
    }

    #[test]
    fn is_not_yaojiupai_2m() {
        let t = Tile::try_from(1).unwrap();
        assert!(!t.is_yaojiupai());
    }

    #[test]
    fn is_hongbaopai_0m() {
        let t = Tile::try_from(34).unwrap();
        assert!(t.is_hongbaopai());
    }

    #[test]
    fn is_not_hongbaopai_7z() {
        let t = Tile::try_from(33).unwrap();
        assert!(!t.is_hongbaopai());
    }

    #[test]
    fn normalize_hongbaopai_0m_to_5m() {
        let t_0m = Tile::try_from(34).unwrap();
        let t_5m = Tile::try_from(4).unwrap();
        assert_eq!(t_0m.normalize_hongbaopai(), t_5m);
    }

    #[test]
    fn normalize_hongbaopai_5m_to_5m() {
        let t_5m = Tile::try_from(4).unwrap();
        assert_eq!(t_5m.normalize_hongbaopai(), t_5m);
    }

    #[test]
    fn normalize_hongbaopai_1m_to_1m() {
        let t_1m = Tile::try_from(0).unwrap();
        assert_eq!(t_1m.normalize_hongbaopai(), t_1m);
    }

    #[test]
    fn decorate_hongbaopai_5p_to_0p() {
        let t_5p = Tile::try_from(13).unwrap();
        let t_0p = Tile::try_from(35).unwrap();
        assert_eq!(t_5p.decorate_hongbaopai(), t_0p);
    }

    #[test]
    fn decorate_hongbaopai_0p_to_0p() {
        let t_0p = Tile::try_from(35).unwrap();
        assert_eq!(t_0p.decorate_hongbaopai(), t_0p);
    }

    #[test]
    fn decorate_hongbaopai_7z_to_7z() {
        let t_7z = Tile::try_from(33).unwrap();
        assert_eq!(t_7z.decorate_hongbaopai(), t_7z);
    }
}
