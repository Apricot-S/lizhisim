// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::common::{Bipai, MAX_NUM_BAOPAI, MAX_TILE_COPIES, NUM_HAND_TILES, NUM_WANGPAI};
use crate::tile::{Tile, TileError};
use crate::{t, tu8, tuz};
use rand::Rng;
use rand::seq::SliceRandom;
use thiserror::Error;

const NUM_BIPAI_TILES: usize = (9 * 3 + 7) * MAX_TILE_COPIES as usize;
const NUM_LINGSHANGPAI: usize = 4;
const RED_5M_INDEX: usize = tuz!(5m) * MAX_TILE_COPIES as usize;
const RED_5P_INDEX: usize = tuz!(5p) * MAX_TILE_COPIES as usize;
const RED_5S_INDEX: usize = tuz!(5s) * MAX_TILE_COPIES as usize;

const NUM_ZIMOPAI: usize = NUM_BIPAI_TILES - NUM_HAND_TILES * 4 - NUM_WANGPAI;
const FIRST_ZIMO_INDEX: usize = NUM_HAND_TILES * 4;

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
    left_tile_count: u8,
    baopai_count: u8,
    zimo_index: usize,
    lingshangzimo_count: usize,
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

        Self {
            tiles,
            left_tile_count: NUM_ZIMOPAI as u8,
            baopai_count: 1,
            zimo_index: FIRST_ZIMO_INDEX,
            lingshangzimo_count: 0,
        }
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

        let config_ = HongbaopaiConfig::new(num_0m, num_0p, num_0s)
            .expect("The number of tiles has already been checked, so there is no error.");

        if config_ != *config {
            return Err(Bipai4pError::HongbaopaiConfigMismatch {
                expected: config.clone(),
                found: config_,
            });
        }

        Ok(Bipai4p {
            tiles,
            left_tile_count: NUM_ZIMOPAI as u8,
            baopai_count: 1,
            zimo_index: FIRST_ZIMO_INDEX,
            lingshangzimo_count: 0,
        })
    }
}

impl Bipai for Bipai4p {
    fn left_tile_count(&self) -> u8 {
        self.left_tile_count
    }

    fn baopai_count(&self) -> u8 {
        self.baopai_count
    }

    fn baopai_indicators(&self) -> Vec<Tile> {
        // The top and bottom of each stack (幢) are reversed in the dead wall (王牌).
        (0..self.baopai_count as usize)
            .map(move |i| NUM_BIPAI_TILES - NUM_LINGSHANGPAI - 2 * i - 1)
            .map(|pos| self.tiles[pos])
            .collect::<Vec<_>>()
    }

    fn libaopai_indicators(&self) -> Vec<Tile> {
        // The top and bottom of each stack (幢) are reversed in the dead wall (王牌).
        (0..self.baopai_count as usize)
            .map(move |i| NUM_BIPAI_TILES - NUM_LINGSHANGPAI - 2 * i - 1 - 1)
            .map(|pos| self.tiles[pos])
            .collect::<Vec<_>>()
    }

    fn qipai(&self, player_index: usize) -> [Tile; NUM_HAND_TILES] {
        debug_assert!(player_index < 4);
        debug_assert_eq!(self.left_tile_count, NUM_ZIMOPAI as u8);
        debug_assert_eq!(self.baopai_count, 1);
        debug_assert_eq!(self.zimo_index, FIRST_ZIMO_INDEX);
        debug_assert_eq!(self.lingshangzimo_count, 0);

        (0..3)
            .flat_map(|i| (0..4).map(move |j| i * 16 + player_index * 4 + j))
            .chain(std::iter::once(48 + player_index))
            .map(|pos| self.tiles[pos])
            .collect::<Vec<_>>()
            .try_into()
            .expect("hand must be exactly 13 tiles")
    }

    fn zimo(&mut self) -> Tile {
        debug_assert!(self.left_tile_count > 0);

        let t = self.tiles[self.zimo_index];
        self.left_tile_count -= 1;
        self.zimo_index += 1;
        t
    }

    fn lingshangzimo(&mut self) -> Tile {
        debug_assert!(self.left_tile_count > 0);
        debug_assert!(self.lingshangzimo_count < NUM_LINGSHANGPAI);

        // The top and bottom of each stack (幢) are reversed in the dead wall (王牌).
        let t = self.tiles[NUM_BIPAI_TILES - self.lingshangzimo_count - 1];
        self.left_tile_count -= 1;
        self.lingshangzimo_count += 1;
        t
    }

    fn kaigang(&mut self) {
        debug_assert!((self.baopai_count as usize) < MAX_NUM_BAOPAI);
        debug_assert!(self.lingshangzimo_count <= NUM_LINGSHANGPAI);

        self.baopai_count += 1
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

    fn get_bipai_for_test() -> Bipai4p {
        #[rustfmt::skip]
        let tiles = [
            tu8!(5s), tu8!(1p), tu8!(3s), tu8!(5m), tu8!(2p), tu8!(7z), tu8!(7p), tu8!(2m), tu8!(4z), tu8!(2z), tu8!(1z), tu8!(7s),
            tu8!(4z), tu8!(5z), tu8!(3p), tu8!(7p), tu8!(9p), tu8!(3z), tu8!(3s), tu8!(8s), tu8!(9m), tu8!(1z), tu8!(4p), tu8!(3p),
            tu8!(4z), tu8!(7p), tu8!(6s), tu8!(6p), tu8!(5z), tu8!(8p), tu8!(7s), tu8!(6z), tu8!(6z), tu8!(4z), tu8!(8m), tu8!(2z),
            tu8!(7s), tu8!(7z), tu8!(1p), tu8!(8m), tu8!(2s), tu8!(1p), tu8!(4p), tu8!(2z), tu8!(9s), tu8!(7p), tu8!(3s), tu8!(4m),
            tu8!(1m), tu8!(4p), tu8!(7m), tu8!(3p), tu8!(4s),

            tu8!(1s), tu8!(0m), tu8!(3s), tu8!(7m), tu8!(8s), tu8!(4s), tu8!(7z), tu8!(8s), tu8!(2m), tu8!(2z), tu8!(6m), tu8!(4m),
            tu8!(9s), tu8!(5s), tu8!(5p), tu8!(5z), tu8!(2s), tu8!(1m), tu8!(2s), tu8!(3z), tu8!(5m), tu8!(4s), tu8!(1s), tu8!(8m),
            tu8!(3m), tu8!(6p), tu8!(7z), tu8!(6s), tu8!(0s), tu8!(3m), tu8!(4s), tu8!(1p), tu8!(0p), tu8!(8s), tu8!(9p), tu8!(8p),
            tu8!(5z), tu8!(3z), tu8!(9p), tu8!(2p), tu8!(6p), tu8!(8p), tu8!(9s), tu8!(2s), tu8!(2m), tu8!(6m), tu8!(4m), tu8!(7s),
            tu8!(5p), tu8!(3p), tu8!(1z), tu8!(4p), tu8!(9m), tu8!(3m), tu8!(9s), tu8!(3m), tu8!(6s), tu8!(6z), tu8!(5s), tu8!(6z),
            tu8!(5p), tu8!(9m), tu8!(1s), tu8!(1m), tu8!(2p), tu8!(1z), tu8!(5m), tu8!(1s), tu8!(2p),

            tu8!(8m), tu8!(6s), tu8!(6m), tu8!(7m), tu8!(6m), tu8!(1m), tu8!(3z), tu8!(9p), tu8!(9m), tu8!(7m), tu8!(4m), tu8!(2m),
            tu8!(6p), tu8!(8p),
        ];
        let config = HongbaopaiConfig::new(1, 1, 1).unwrap();
        Bipai4p::from_slice(&tiles, &config).unwrap()
    }

    #[test]
    fn left_tile_count_before_zimo() {
        let bipai = get_bipai_for_test();

        assert_eq!(bipai.left_tile_count(), 70);
    }

    #[test]
    fn left_tile_count_after_first_zimo() {
        let mut bipai = get_bipai_for_test();

        let _ = bipai.zimo();
        assert_eq!(bipai.left_tile_count(), 69);
    }

    #[test]
    fn left_tile_count_no_tiles() {
        let mut bipai = get_bipai_for_test();

        for _ in 0..70 {
            let _ = bipai.zimo();
        }

        assert_eq!(bipai.left_tile_count(), 0);
    }

    #[test]
    fn baopai_indicators_no_kaigang() {
        let bipai = get_bipai_for_test();
        assert_eq!(bipai.baopai_indicators(), vec![t!(7m)]);
    }

    #[test]
    fn baopai_indicators_4_kaigang() {
        let mut bipai = get_bipai_for_test();
        for _ in 0..4 {
            bipai.kaigang();
        }
        assert_eq!(
            bipai.baopai_indicators(),
            vec![t!(7m), t!(9p), t!(1m), t!(7m), t!(6s)]
        );
    }

    #[test]
    fn libaopai_indicators_no_kaigang() {
        let bipai = get_bipai_for_test();
        assert_eq!(bipai.libaopai_indicators(), vec![t!(9m)]);
    }

    #[test]
    fn libaopai_indicators_4_kaigang() {
        let mut bipai = get_bipai_for_test();
        for _ in 0..4 {
            bipai.kaigang();
        }
        assert_eq!(
            bipai.libaopai_indicators(),
            vec![t!(9m), t!(3z), t!(6m), t!(6m), t!(8m)]
        );
    }

    #[test]
    fn qipai_index_0() {
        let bipai = get_bipai_for_test();
        let bingpai = bipai.qipai(0);

        #[rustfmt::skip]
        let expected = [
            t!(5s), t!(1p), t!(3s), t!(5m),
            t!(9p), t!(3z), t!(3s), t!(8s),
            t!(6z), t!(4z), t!(8m), t!(2z),
            t!(1m),
        ];
        assert_eq!(bingpai, expected);
    }

    #[test]
    fn qipai_index_3() {
        let bipai = get_bipai_for_test();
        let bingpai = bipai.qipai(3);

        #[rustfmt::skip]
        let expected = [
            t!(4z), t!(5z), t!(3p), t!(7p),
            t!(5z), t!(8p), t!(7s), t!(6z),
            t!(9s), t!(7p), t!(3s), t!(4m),
            t!(3p),
            
        ];
        assert_eq!(bingpai, expected);
    }

    #[test]
    fn zimo_first() {
        let mut bipai = get_bipai_for_test();

        let zimopai = bipai.zimo();
        assert_eq!(zimopai, t!(4s));
    }

    #[test]
    fn zimo_second() {
        let mut bipai = get_bipai_for_test();

        let _ = bipai.zimo();
        let zimopai = bipai.zimo();
        assert_eq!(zimopai, t!(1s));
    }

    #[test]
    fn lingshangzimo_first() {
        let mut bipai = get_bipai_for_test();

        let zimopai = bipai.lingshangzimo();
        assert_eq!(zimopai, t!(8p));
    }

    #[test]
    fn lingshangzimo_last() {
        let mut bipai = get_bipai_for_test();

        for _ in 0..3 {
            let _ = bipai.lingshangzimo();
        }

        let zimopai = bipai.lingshangzimo();
        assert_eq!(zimopai, t!(4m));
    }

    #[test]
    fn lingshangzimo_not_affect_zimo_index() {
        let mut bipai = get_bipai_for_test();

        let _ = bipai.lingshangzimo();
        let zimopai = bipai.zimo();
        assert_eq!(zimopai, t!(4s));
    }
}
