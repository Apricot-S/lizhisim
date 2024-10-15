use crate::{matches_tile_index, tile, tile_index};
use std::cmp::Ordering;
use std::fmt;
use thiserror::Error;

const NUM_SHUPAI_COLOR: u8 = 3;
pub(crate) const NUM_SHUPAI_RANK: u8 = 9;
const NUM_SHUPAI: u8 = NUM_SHUPAI_COLOR * NUM_SHUPAI_RANK;
const NUM_FENGPAI: u8 = 4;
const NUM_SANYUANPAI: u8 = 3;
const NUM_HONGBAOPAI: u8 = 3;

pub(crate) const NUM_TILE_INDEX_WITHOUT_HONGBAOPAI: usize =
    (NUM_SHUPAI + NUM_FENGPAI + NUM_SANYUANPAI) as usize;
pub(crate) const NUM_TILE_INDEX: usize =
    NUM_TILE_INDEX_WITHOUT_HONGBAOPAI + NUM_HONGBAOPAI as usize;
pub(crate) const NUM_SAME_TILE: u8 = 4;

#[derive(Error, Debug)]
pub enum ParseTileError {
    #[error("Invalid index: {0}")]
    InvalidIndex(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile(u8);

impl Tile {
    #[inline]
    #[must_use]
    pub const fn new_unchecked(index: u8) -> Self {
        Self(index)
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(&self) -> &u8 {
        &self.0
    }

    #[inline]
    #[must_use]
    pub const fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    #[must_use]
    pub const fn is_hongbaopai(&self) -> bool {
        matches_tile_index!(self.0, 0m | 0p | 0s)
    }

    #[inline]
    #[must_use]
    pub const fn is_laotoupai(&self) -> bool {
        matches_tile_index!(self.0, 1m | 9m | 1p | 9p | 1s | 9s)
    }

    #[inline]
    #[must_use]
    pub const fn is_zipai(&self) -> bool {
        matches_tile_index!(self.0, 1z | 2z | 3z | 4z | 5z | 6z | 7z)
    }

    #[inline]
    #[must_use]
    pub const fn is_yaojiupai(&self) -> bool {
        matches_tile_index!(
            self.0,
            1m | 9m | 1p | 9p | 1s | 9s | 1z | 2z | 3z | 4z | 5z | 6z | 7z
        )
    }

    #[inline]
    #[must_use]
    pub const fn normalize_hongbaopai(self) -> Self {
        match self.0 {
            tile_index!(0m) => tile!(5m),
            tile_index!(0p) => tile!(5p),
            tile_index!(0s) => tile!(5s),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    pub const fn decorate_hongbaopai(self) -> Self {
        match self.0 {
            tile_index!(5m) => tile!(0m),
            tile_index!(5p) => tile!(0p),
            tile_index!(5s) => tile!(0s),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    pub const fn prev(self) -> Self {
        let normalize_tile_index = *self.normalize_hongbaopai().as_u8();
        let color = normalize_tile_index / NUM_SHUPAI_RANK;
        let rank = normalize_tile_index % NUM_SHUPAI_RANK;

        if color < NUM_SHUPAI_COLOR {
            // shupai
            Self::new_unchecked(
                color * NUM_SHUPAI_RANK + ((NUM_SHUPAI_RANK + rank) - 1) % NUM_SHUPAI_RANK,
            )
        } else if rank < NUM_FENGPAI {
            // fengpai
            Self::new_unchecked(NUM_SHUPAI + ((NUM_FENGPAI + rank) - 1) % NUM_FENGPAI)
        } else {
            // sanyuanpai
            Self::new_unchecked(
                NUM_SHUPAI
                    + NUM_FENGPAI
                    + ((NUM_SANYUANPAI + rank) - NUM_FENGPAI - 1) % NUM_SANYUANPAI,
            )
        }
    }

    #[inline]
    #[must_use]
    pub const fn next(self) -> Self {
        let normalize_tile_index = *self.normalize_hongbaopai().as_u8();
        let color = normalize_tile_index / NUM_SHUPAI_RANK;
        let rank = normalize_tile_index % NUM_SHUPAI_RANK;

        if color < NUM_SHUPAI_COLOR {
            // shupai
            Self::new_unchecked(color * NUM_SHUPAI_RANK + (rank + 1) % NUM_SHUPAI_RANK)
        } else if rank < NUM_FENGPAI {
            // fengpai
            Self::new_unchecked(NUM_SHUPAI + (rank + 1) % NUM_FENGPAI)
        } else {
            // sanyuanpai
            Self::new_unchecked(
                NUM_SHUPAI + NUM_FENGPAI + (rank - NUM_FENGPAI + 1) % NUM_SANYUANPAI,
            )
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseTileError;

    #[inline]
    #[must_use]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < NUM_TILE_INDEX as u8 {
            Ok(Self(value))
        } else {
            Err(ParseTileError::InvalidIndex(value))
        }
    }
}

impl PartialOrd for Tile {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tile {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        #[inline]
        fn convert_to_cmp(index: &u8) -> u8 {
            match index {
                // Due to Rust's language specification,
                // `tile_index!` could not be used.
                0..=3 => *index,      // 1m - 4m
                34 => 4,              // 0m (r5m)
                4..=12 => index + 1,  // 5m - 4p
                35 => 13 + 1,         // 0p (r5p)
                13..=21 => index + 2, // 5p - 4s
                36 => 22 + 2,         // 0s (r5s)
                22..=33 => index + 3, // 5s - 7z (hongzhong)
                _ => panic!("Invalid tile index"),
            }
        }
        convert_to_cmp(self.as_u8()).cmp(&convert_to_cmp(other.as_u8()))
    }
}

const TILE_NAMES: [&str; NUM_TILE_INDEX] = [
    "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", // wanzi
    "1p", "2p", "3p", "4p", "5p", "6p", "7p", "8p", "9p", // bingzi
    "1s", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", // suozi
    "1z", "2z", "3z", "4z", "5z", "6z", "7z", // zipai
    "0m", "0p", "0s", // hongbaopai
];

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(TILE_NAMES[self.to_usize()])
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
