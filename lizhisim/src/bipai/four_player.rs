// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::common::MAX_TILE_COPIES;
use crate::tile::{Tile, TileError};
use crate::{t, tu8, tuz};
use rand::Rng;
use rand::seq::SliceRandom;
use thiserror::Error;

const NUM_BIPAI_TILES: usize = (9 * 3 + 7) * MAX_TILE_COPIES as usize;
const RED_5M_INDEX: usize = tuz!(5m) * MAX_TILE_COPIES as usize;
const RED_5P_INDEX: usize = tuz!(5p) * MAX_TILE_COPIES as usize;
const RED_5S_INDEX: usize = tuz!(5s) * MAX_TILE_COPIES as usize;

#[derive(Debug, Clone, PartialEq, Eq)]
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
            Ok(Self { m, p, s })
        } else {
            Err(HongbaopaiConfigError { m, p, s })
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

#[derive(Debug)]
pub(crate) struct Bipai4p {
    tiles: [Tile; NUM_BIPAI_TILES],
}

#[derive(Debug, Error)]
pub(crate) enum Bipai4pError {
    #[error("invalid number of tiles: {0}")]
    InvalidLength(usize),
    #[error(transparent)]
    Tile(#[from] TileError),
    #[error("tile {0:?} appears {1} times instead of 4")]
    InvalidTileCount(Tile, u8),
    #[error("red five config mismatch: expected {expected:?}, found {found:?}")]
    HongbaopaiConfigMismatch {
        expected: HongbaopaiConfig,
        found: HongbaopaiConfig,
    },
}

impl Bipai4p {
    pub(crate) fn new(rng: &mut impl Rng, config: &HongbaopaiConfig) -> Self {
        #[rustfmt::skip]
        let mut tiles = [
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

        Self::apply_hongbaopai_config(&mut tiles, config);
        tiles.shuffle(rng);
        Self { tiles }
    }

    fn apply_hongbaopai_config(tiles: &mut [Tile; NUM_BIPAI_TILES], config: &HongbaopaiConfig) {
        Self::replace_with_hongbaopai(tiles, RED_5M_INDEX, t!(0m), config.m());
        Self::replace_with_hongbaopai(tiles, RED_5P_INDEX, t!(0p), config.p());
        Self::replace_with_hongbaopai(tiles, RED_5S_INDEX, t!(0s), config.s());
    }

    fn replace_with_hongbaopai(
        tiles: &mut [Tile; NUM_BIPAI_TILES],
        base_index: usize,
        red: Tile,
        count: u8,
    ) {
        for i in 0..count {
            let idx = base_index + i as usize;
            tiles[idx] = red;
        }
    }

    pub(crate) fn from_slice(
        bipai: &[u8],
        config: &HongbaopaiConfig,
    ) -> Result<Self, Bipai4pError> {
        if bipai.len() != NUM_BIPAI_TILES {
            return Err(Bipai4pError::InvalidLength(bipai.len()));
        }

        let tiles: [Tile; NUM_BIPAI_TILES] = bipai
            .iter()
            .map(|&t| Tile::try_from(t))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<Tile>| Bipai4pError::InvalidLength(v.len()))?;

        let mut counts = [0u8; 34];
        let mut num_0m = 0;
        let mut num_0p = 0;
        let mut num_0s = 0;
        for tile in tiles {
            counts[tile.normalize_hongbaopai().to_usize()] += 1;

            match tile.as_u8() {
                tu8!(0m) => num_0m += 1,
                tu8!(0p) => num_0p += 1,
                tu8!(0s) => num_0s += 1,
                _ => (),
            }
        }

        if let Some((i, &count)) = counts
            .iter()
            .enumerate()
            .find(|&(_, &c)| c != MAX_TILE_COPIES)
        {
            return Err(Bipai4pError::InvalidTileCount(
                Tile::try_from(i as u8)?,
                count,
            ));
        }

        // The number of tiles has already been checked, so there is no error.
        let config_ = HongbaopaiConfig::new(num_0m, num_0p, num_0s).unwrap();

        if config_ != *config {
            return Err(Bipai4pError::HongbaopaiConfigMismatch {
                expected: config.clone(),
                found: config_,
            });
        }

        Ok(Bipai4p { tiles })
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
        let config = HongbaopaiConfig::new(0, 1, 2).unwrap();
        let bipai = Bipai4p::new(&mut rng, &config);

        let mut counts = [0u8; 34];
        let mut num_0m = 0;
        let mut num_0p = 0;
        let mut num_0s = 0;
        for tile in bipai.tiles {
            counts[tile.normalize_hongbaopai().to_usize()] += 1;

            match tile.as_u8() {
                tu8!(0m) => num_0m += 1,
                tu8!(0p) => num_0p += 1,
                tu8!(0s) => num_0s += 1,
                _ => (),
            }
        }

        assert_eq!(counts, [4; 34]);
        assert_eq!(num_0m, 0);
        assert_eq!(num_0p, 1);
        assert_eq!(num_0s, 2);
    }

    #[test]
    fn from_slice_valid() {
        let mut tiles = (0..136).map(|t| t / 4).collect::<Vec<u8>>();
        tiles[13 * 4] = 35;
        tiles[22 * 4] = 36;
        tiles[22 * 4 + 1] = 36;
        let config = HongbaopaiConfig::new(0, 1, 2).unwrap();
        let bipai = Bipai4p::from_slice(&tiles, &config).unwrap();

        let mut counts = [0u8; 34];
        let mut num_0m = 0;
        let mut num_0p = 0;
        let mut num_0s = 0;
        for tile in bipai.tiles {
            counts[tile.normalize_hongbaopai().to_usize()] += 1;

            match tile.as_u8() {
                tu8!(0m) => num_0m += 1,
                tu8!(0p) => num_0p += 1,
                tu8!(0s) => num_0s += 1,
                _ => (),
            }
        }

        assert_eq!(counts, [4; 34]);
        assert_eq!(num_0m, 0);
        assert_eq!(num_0p, 1);
        assert_eq!(num_0s, 2);
    }

    #[test]
    fn from_slice_invalid_135_tiles() {
        let tiles = (0..135).map(|t| t / 4).collect::<Vec<u8>>();
        let config = HongbaopaiConfig::new(0, 0, 0).unwrap();
        let err = Bipai4p::from_slice(&tiles, &config).unwrap_err();

        assert!(matches!(err, Bipai4pError::InvalidLength(135)));
    }

    #[test]
    fn from_slice_invalid_137_tiles() {
        let tiles = (0..137).map(|t| t / 4).collect::<Vec<u8>>();
        let config = HongbaopaiConfig::new(0, 0, 0).unwrap();
        let err = Bipai4p::from_slice(&tiles, &config).unwrap_err();

        assert!(matches!(err, Bipai4pError::InvalidLength(137)));
    }

    #[test]
    fn from_slice_invalid_tiles_id() {
        let mut tiles = (0..136).map(|t| t / 4).collect::<Vec<u8>>();
        tiles[135] = 37;
        let config = HongbaopaiConfig::new(0, 0, 0).unwrap();
        let err = Bipai4p::from_slice(&tiles, &config).unwrap_err();

        assert!(matches!(err, Bipai4pError::Tile(TileError::OutOfRange(37))));
    }

    #[test]
    fn from_slice_invalid_1m_5_copies() {
        let mut tiles = (0..136).map(|t| t / 4).collect::<Vec<u8>>();
        tiles[135] = 0;
        let config = HongbaopaiConfig::new(0, 0, 0).unwrap();
        let err = Bipai4p::from_slice(&tiles, &config).unwrap_err();

        if let Bipai4pError::InvalidTileCount(tile, 5) = err {
            assert_eq!(tile, t!(1m));
        } else {
            panic!("unexpected error: {:?}", err);
        }
    }

    #[test]
    fn from_slice_invalid_config_mismatch() {
        let tiles = (0..136).map(|t| t / 4).collect::<Vec<u8>>();
        let config = HongbaopaiConfig::new(0, 1, 2).unwrap();
        let err = Bipai4p::from_slice(&tiles, &config).unwrap_err();

        if let Bipai4pError::HongbaopaiConfigMismatch { expected, found } = err {
            assert_eq!(expected, config);
            assert_eq!(found, HongbaopaiConfig::new(0, 0, 0).unwrap());
        } else {
            panic!("unexpected error: {:?}", err);
        }
    }
}
