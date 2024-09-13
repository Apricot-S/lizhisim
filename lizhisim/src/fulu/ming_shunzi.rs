use super::ming_mianzi::{Mianzi, MingMianzi};
use super::tajia::Tajia;
use crate::matches_tile_index;
use crate::tile::{Tile, NUM_SHUPAI_RANK};
use anyhow::{ensure, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ClaimedTilePosition {
    Low,
    Middle,
    High,
}

#[derive(Debug, Clone)]
pub struct MingShunzi {
    tiles: [Tile; 3],
    discarder: Tajia,
    claimed_tile_position: ClaimedTilePosition,
    shiti_jin: Option<Tile>,
}

impl MingShunzi {
    #[must_use]
    pub fn new(claimed_tile: Tile, dazi: [Tile; 2]) -> Result<Self> {
        let normalized_claimed_tile = claimed_tile.normalize_hongbaopai();
        let mut normalized_shunzi = [
            normalized_claimed_tile,
            dazi[0].normalize_hongbaopai(),
            dazi[1].normalize_hongbaopai(),
        ];

        let colors = normalized_shunzi.map(|t| *t.as_u8() / NUM_SHUPAI_RANK);
        ensure!(
            (colors[0] == colors[1]) && (colors[1] == colors[2]),
            "All tiles to Chii must be of the same colors.: {:?}, {:?}, {:?}",
            claimed_tile,
            dazi[0],
            dazi[1],
        );

        ensure!(
            !normalized_claimed_tile.is_zipai(),
            "It is not possible to Chii with Honors.: {:?}, {:?}, {:?}",
            claimed_tile,
            dazi[0],
            dazi[1],
        );

        normalized_shunzi.sort();
        ensure!(
            (normalized_shunzi[0].next() == normalized_shunzi[1])
                && (normalized_shunzi[1].next() == normalized_shunzi[2]),
            "Tiles do not form a valid Chii: {:?}, {:?}, {:?}",
            claimed_tile,
            dazi[0],
            dazi[1],
        );

        let (claimed_tile_position, shiti_jin) = match normalized_claimed_tile {
            tile if tile == normalized_shunzi[0] => {
                let shiti_jin = if matches_tile_index!(tile.as_u8(), 7m | 7p | 7s) {
                    // In case of { claimed_tile: 7x, dazi: [8x, 9x] }
                    None
                } else {
                    Some(Tile::new_unchecked(tile.as_u8() + 3))
                };
                (ClaimedTilePosition::Low, shiti_jin)
            }
            tile if tile == normalized_shunzi[2] => {
                let shiti_jin = if matches_tile_index!(tile.as_u8(), 3m | 3p | 3s) {
                    // In case of { claimed_tile: 3x, dazi: [1x, 2x] }
                    None
                } else {
                    Some(Tile::new_unchecked(tile.as_u8() - 3))
                };
                (ClaimedTilePosition::High, shiti_jin)
            }
            _ => (ClaimedTilePosition::Middle, None),
        };

        let mut sorted_dazi = dazi;
        sorted_dazi.sort();
        Ok(Self {
            tiles: [claimed_tile, sorted_dazi[0], sorted_dazi[1]],
            discarder: Tajia::Shangjia,
            claimed_tile_position,
            shiti_jin,
        })
    }

    #[inline]
    #[must_use]
    pub fn claimed_tile_position(&self) -> &ClaimedTilePosition {
        &self.claimed_tile_position
    }

    #[inline]
    #[must_use]
    pub fn shiti_jin(&self) -> &Option<Tile> {
        &self.shiti_jin
    }
}

impl Mianzi<3> for MingShunzi {
    #[inline]
    #[must_use]
    fn tiles(&self) -> &[Tile; 3] {
        &self.tiles
    }
}

impl MingMianzi<3> for MingShunzi {
    #[inline]
    #[must_use]
    fn claimed_tile(&self) -> &Tile {
        &self.tiles[0]
    }

    #[inline]
    #[must_use]
    fn discarder(&self) -> &Tajia {
        &self.discarder
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tile, tile_array};

    #[test]
    fn test_valid_ming_shunzi() {
        let shunzi_123m = MingShunzi::new(tile!(1m), tile_array![2m, 3m]).unwrap();
        let shunzi_132m = MingShunzi::new(tile!(1m), tile_array![3m, 2m]).unwrap();

        assert_eq!(*shunzi_123m.tiles(), tile_array![1m, 2m, 3m]);
        assert_eq!(*shunzi_123m.tiles(), *shunzi_132m.tiles());
        assert_eq!(*shunzi_123m.discarder(), Tajia::Shangjia);
        assert_eq!(shunzi_123m.shiti_jin().unwrap(), tile!(4m));
    }

    #[test]
    fn test_valid_ming_shunzi_0() {
        let shunzi_034m = MingShunzi::new(tile!(0m), tile_array![3m, 4m]).unwrap();
        let shunzi_043m = MingShunzi::new(tile!(0m), tile_array![4m, 3m]).unwrap();

        assert_eq!(*shunzi_034m.tiles(), tile_array![0m, 3m, 4m]);
        assert_eq!(*shunzi_034m.tiles(), *shunzi_043m.tiles());
        assert_eq!(*shunzi_034m.discarder(), Tajia::Shangjia);
        assert_eq!(shunzi_034m.shiti_jin().unwrap(), tile!(2m));
    }

    #[test]
    fn test_invalid_ming_shunzi() {
        _ = MingShunzi::new(tile!(1z), tile_array![2m, 3m]).unwrap_err();
        _ = MingShunzi::new(tile!(1m), tile_array![2m, 3z]).unwrap_err();

        _ = MingShunzi::new(tile!(1m), tile_array![1m, 1m]).unwrap_err();
        _ = MingShunzi::new(tile!(1m), tile_array![2m, 4m]).unwrap_err();
        _ = MingShunzi::new(tile!(1m), tile_array![3m, 4m]).unwrap_err();

        _ = MingShunzi::new(tile!(1m), tile_array![2m, 3p]).unwrap_err();
        _ = MingShunzi::new(tile!(1p), tile_array![2m, 3m]).unwrap_err();
    }
}
