// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::tile::{Tile, TileError};
use crate::{t, tuz};
use rand::Rng;
use rand::seq::SliceRandom;
use thiserror::Error;

const MAX_TILE_COPIES: u8 = 4;
const NUM_WANGPAI: usize = 14;
const NUM_LINGSHANGPAI: usize = 4;
const MAX_GANG_COUNT: usize = 4;
const MAX_NUM_BAOPAI: usize = MAX_GANG_COUNT + 1;

const NUM_BIPAI_TILES: usize = (9 * 3 + 7) * MAX_TILE_COPIES as usize;
const RED_5M_INDEX: usize = tuz!(5m) * MAX_TILE_COPIES as usize;
const RED_5P_INDEX: usize = tuz!(5p) * MAX_TILE_COPIES as usize;
const RED_5S_INDEX: usize = tuz!(5s) * MAX_TILE_COPIES as usize;

#[rustfmt::skip]
const INITIAL_BIPAI: [Tile; NUM_BIPAI_TILES] = [
    t!(1m), t!(1m), t!(1m), t!(1m),
    t!(2m), t!(2m), t!(2m), t!(2m),
    t!(3m), t!(3m), t!(3m), t!(3m),
    t!(4m), t!(4m), t!(4m), t!(4m),
    t!(5m), t!(5m), t!(5m), t!(5m),
    t!(6m), t!(6m), t!(6m), t!(6m),
    t!(7m), t!(7m), t!(7m), t!(7m),
    t!(8m), t!(8m), t!(8m), t!(8m),
    t!(9m), t!(9m), t!(9m), t!(9m),

    t!(1p), t!(1p), t!(1p), t!(1p),
    t!(2p), t!(2p), t!(2p), t!(2p),
    t!(3p), t!(3p), t!(3p), t!(3p),
    t!(4p), t!(4p), t!(4p), t!(4p),
    t!(5p), t!(5p), t!(5p), t!(5p),
    t!(6p), t!(6p), t!(6p), t!(6p),
    t!(7p), t!(7p), t!(7p), t!(7p),
    t!(8p), t!(8p), t!(8p), t!(8p),
    t!(9p), t!(9p), t!(9p), t!(9p),

    t!(1s), t!(1s), t!(1s), t!(1s),
    t!(2s), t!(2s), t!(2s), t!(2s),
    t!(3s), t!(3s), t!(3s), t!(3s),
    t!(4s), t!(4s), t!(4s), t!(4s),
    t!(5s), t!(5s), t!(5s), t!(5s),
    t!(6s), t!(6s), t!(6s), t!(6s),
    t!(7s), t!(7s), t!(7s), t!(7s),
    t!(8s), t!(8s), t!(8s), t!(8s),
    t!(9s), t!(9s), t!(9s), t!(9s),

    t!(1z), t!(1z), t!(1z), t!(1z),
    t!(2z), t!(2z), t!(2z), t!(2z),
    t!(3z), t!(3z), t!(3z), t!(3z),
    t!(4z), t!(4z), t!(4z), t!(4z),
    t!(5z), t!(5z), t!(5z), t!(5z),
    t!(6z), t!(6z), t!(6z), t!(6z),
    t!(7z), t!(7z), t!(7z), t!(7z),
];

#[derive(Debug)]
pub(crate) struct HongbaopaiConfig {
    m: u8,
    p: u8,
    s: u8,
}

#[derive(Debug, Error)]
#[error("invalid number of red fives: m: {m}, p: {p}, s: {s}")]
pub(crate) struct HongbaopaiConfigError {
    pub(crate) m: u8,
    pub(crate) p: u8,
    pub(crate) s: u8,
}

impl HongbaopaiConfig {
    #[inline]
    pub(crate) const fn new(m: u8, p: u8, s: u8) -> Result<Self, HongbaopaiConfigError> {
        if m <= MAX_TILE_COPIES && p <= MAX_TILE_COPIES && s <= MAX_TILE_COPIES {
            Ok(Self { m: m, p: p, s: s })
        } else {
            Err(HongbaopaiConfigError { m: m, p: p, s: s })
        }
    }

    #[inline]
    #[must_use]
    pub(crate) const fn m(&self) -> u8 {
        self.m
    }

    #[inline]
    #[must_use]
    pub(crate) const fn p(&self) -> u8 {
        self.p
    }

    #[inline]
    #[must_use]
    pub(crate) const fn s(&self) -> u8 {
        self.s
    }
}

/// 壁牌: Wall.
#[derive(Debug)]
pub(crate) struct Bipai {
    tiles: [Tile; NUM_BIPAI_TILES],
}

#[derive(Debug, Error)]
pub(crate) enum BipaiError {
    #[error("invalid number of tiles: {0}")]
    WrongLength(usize),
    #[error(transparent)]
    Tile(#[from] TileError),
    #[error("tile {0:?} appears {1} times instead of 4")]
    WrongMultiplicity(Tile, u8),
}

impl Bipai {
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        let mut tiles = INITIAL_BIPAI;
        tiles.shuffle(rng);
        Self { tiles: tiles }
    }

    pub(crate) fn from_slice(bipai: &[u8]) -> Result<Self, BipaiError> {
        if bipai.len() != NUM_BIPAI_TILES {
            return Err(BipaiError::WrongLength(bipai.len()));
        }

        let tiles: [Tile; NUM_BIPAI_TILES] = bipai
            .iter()
            .map(|&t| Tile::try_from(t))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<Tile>| BipaiError::WrongLength(v.len()))?;

        let mut counts = [0u8; 34];
        for tile in tiles {
            counts[tile.normalize_hongbaopai().to_usize()] += 1;
        }

        if let Some((i, &count)) = counts
            .iter()
            .enumerate()
            .find(|&(_, &c)| c != MAX_TILE_COPIES)
        {
            return Err(BipaiError::WrongMultiplicity(
                Tile::try_from(i as u8)?,
                count,
            ));
        }

        Ok(Bipai { tiles })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn hongbaopai_config_valid_all_0() {
        let config = HongbaopaiConfig::new(0, 0, 0).unwrap();
        assert_eq!(config.m(), 0);
        assert_eq!(config.p(), 0);
        assert_eq!(config.s(), 0);
    }

    #[test]
    fn hongbaopai_config_valid_all_4() {
        let config = HongbaopaiConfig::new(4, 4, 4).unwrap();
        assert_eq!(config.m(), 4);
        assert_eq!(config.p(), 4);
        assert_eq!(config.s(), 4);
    }

    #[test]
    fn hongbaopai_config_invalid_m_5() {
        let err = HongbaopaiConfig::new(5, 0, 0).unwrap_err();
        assert_eq!(err.m, 5);
        assert_eq!(err.p, 0);
        assert_eq!(err.s, 0);
    }

    #[test]
    fn new_bipai() {
        let mut rng = StdRng::seed_from_u64(42);
        let bipai = Bipai::new(&mut rng);

        let mut counts = [0u8; 34];
        for tile in bipai.tiles {
            counts[tile.normalize_hongbaopai().to_usize()] += 1;
        }

        assert_eq!(counts, [4; 34]);
    }
}
