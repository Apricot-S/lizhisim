use super::ming_mianzi::{Mianzi, MingMianzi};
use super::tajia::Tajia;
use crate::tile::Tile;
use anyhow::{ensure, Result};

#[derive(Debug, Clone)]
pub struct MingKezi {
    tiles: [Tile; 3],
    discarder: Tajia,
}

impl MingKezi {
    #[must_use]
    pub fn new(claimed_tile: Tile, dazi: [Tile; 2], discarder: Tajia) -> Result<Self> {
        let normalized_claimed_tile = claimed_tile.normalize_hongbaopai();
        ensure!(
            !dazi
                .iter()
                .any(|&t| t.normalize_hongbaopai() != normalized_claimed_tile),
            "Tiles do not form a valid Pon: {:?}, {:?}, {:?}",
            claimed_tile,
            dazi[0],
            dazi[1],
        );

        let sorted_dazi = if dazi[1].is_hongbaopai() {
            [dazi[1], dazi[0]]
        } else {
            dazi
        };

        Ok(Self {
            tiles: [claimed_tile, sorted_dazi[0], sorted_dazi[1]],
            discarder,
        })
    }
}

impl Mianzi<3> for MingKezi {
    #[inline]
    #[must_use]
    fn tiles(&self) -> &[Tile; 3] {
        &self.tiles
    }
}

impl MingMianzi<3> for MingKezi {
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
mod tests {
    use super::*;
    use crate::{tile, tile_array};

    #[test]
    fn test_valid_ming_kezi() {
        let kezi_1m = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Xiajia).unwrap();
        let kezi_1p = MingKezi::new(tile!(1p), tile_array![1p, 1p], Tajia::Xiajia).unwrap();
        let kezi_1s = MingKezi::new(tile!(1s), tile_array![1s, 1s], Tajia::Xiajia).unwrap();
        let kezi_1z = MingKezi::new(tile!(1z), tile_array![1z, 1z], Tajia::Xiajia).unwrap();

        assert_eq!(*kezi_1m.tiles(), tile_array![1m, 1m, 1m]);
        assert_eq!(*kezi_1p.tiles(), tile_array![1p, 1p, 1p]);
        assert_eq!(*kezi_1s.tiles(), tile_array![1s, 1s, 1s]);
        assert_eq!(*kezi_1z.tiles(), tile_array![1z, 1z, 1z]);

        assert_eq!(*kezi_1m.discarder(), *kezi_1p.discarder());
    }

    #[test]
    fn test_valid_ming_kezi_5() {
        let kezi_000m = MingKezi::new(tile!(0m), tile_array![0m, 0m], Tajia::Xiajia).unwrap();
        let kezi_005m = MingKezi::new(tile!(0m), tile_array![0m, 5m], Tajia::Xiajia).unwrap();
        let kezi_050m = MingKezi::new(tile!(0m), tile_array![5m, 0m], Tajia::Xiajia).unwrap();
        let kezi_055m = MingKezi::new(tile!(0m), tile_array![5m, 5m], Tajia::Xiajia).unwrap();
        let kezi_500m = MingKezi::new(tile!(5m), tile_array![0m, 0m], Tajia::Xiajia).unwrap();
        let kezi_505m = MingKezi::new(tile!(5m), tile_array![0m, 5m], Tajia::Xiajia).unwrap();
        let kezi_550m = MingKezi::new(tile!(5m), tile_array![5m, 0m], Tajia::Xiajia).unwrap();
        let kezi_555m = MingKezi::new(tile!(5m), tile_array![5m, 5m], Tajia::Xiajia).unwrap();

        assert_eq!(*kezi_000m.tiles(), tile_array![0m, 0m, 0m]);
        assert_eq!(*kezi_005m.tiles(), tile_array![0m, 0m, 5m]);
        assert_eq!(*kezi_050m.tiles(), tile_array![0m, 0m, 5m]);
        assert_eq!(*kezi_055m.tiles(), tile_array![0m, 5m, 5m]);
        assert_eq!(*kezi_500m.tiles(), tile_array![5m, 0m, 0m]);
        assert_eq!(*kezi_505m.tiles(), tile_array![5m, 0m, 5m]);
        assert_eq!(*kezi_550m.tiles(), tile_array![5m, 0m, 5m]);
        assert_eq!(*kezi_555m.tiles(), tile_array![5m, 5m, 5m]);

        assert_eq!(kezi_005m.tiles(), kezi_050m.tiles());
        assert_eq!(kezi_505m.tiles(), kezi_550m.tiles());

        let kezi_000p = MingKezi::new(tile!(0p), tile_array![0p, 0p], Tajia::Xiajia).unwrap();
        let kezi_005p = MingKezi::new(tile!(0p), tile_array![0p, 5p], Tajia::Xiajia).unwrap();
        let kezi_050p = MingKezi::new(tile!(0p), tile_array![5p, 0p], Tajia::Xiajia).unwrap();
        let kezi_055p = MingKezi::new(tile!(0p), tile_array![5p, 5p], Tajia::Xiajia).unwrap();
        let kezi_500p = MingKezi::new(tile!(5p), tile_array![0p, 0p], Tajia::Xiajia).unwrap();
        let kezi_505p = MingKezi::new(tile!(5p), tile_array![0p, 5p], Tajia::Xiajia).unwrap();
        let kezi_550p = MingKezi::new(tile!(5p), tile_array![5p, 0p], Tajia::Xiajia).unwrap();
        let kezi_555p = MingKezi::new(tile!(5p), tile_array![5p, 5p], Tajia::Xiajia).unwrap();

        assert_eq!(*kezi_000p.tiles(), tile_array![0p, 0p, 0p]);
        assert_eq!(*kezi_005p.tiles(), tile_array![0p, 0p, 5p]);
        assert_eq!(*kezi_050p.tiles(), tile_array![0p, 0p, 5p]);
        assert_eq!(*kezi_055p.tiles(), tile_array![0p, 5p, 5p]);
        assert_eq!(*kezi_500p.tiles(), tile_array![5p, 0p, 0p]);
        assert_eq!(*kezi_505p.tiles(), tile_array![5p, 0p, 5p]);
        assert_eq!(*kezi_550p.tiles(), tile_array![5p, 0p, 5p]);
        assert_eq!(*kezi_555p.tiles(), tile_array![5p, 5p, 5p]);

        assert_eq!(kezi_005p.tiles(), kezi_050p.tiles());
        assert_eq!(kezi_505p.tiles(), kezi_550p.tiles());

        let kezi_000s = MingKezi::new(tile!(0s), tile_array![0s, 0s], Tajia::Xiajia).unwrap();
        let kezi_005s = MingKezi::new(tile!(0s), tile_array![0s, 5s], Tajia::Xiajia).unwrap();
        let kezi_050s = MingKezi::new(tile!(0s), tile_array![5s, 0s], Tajia::Xiajia).unwrap();
        let kezi_055s = MingKezi::new(tile!(0s), tile_array![5s, 5s], Tajia::Xiajia).unwrap();
        let kezi_500s = MingKezi::new(tile!(5s), tile_array![0s, 0s], Tajia::Xiajia).unwrap();
        let kezi_505s = MingKezi::new(tile!(5s), tile_array![0s, 5s], Tajia::Xiajia).unwrap();
        let kezi_550s = MingKezi::new(tile!(5s), tile_array![5s, 0s], Tajia::Xiajia).unwrap();
        let kezi_555s = MingKezi::new(tile!(5s), tile_array![5s, 5s], Tajia::Xiajia).unwrap();

        assert_eq!(*kezi_000s.tiles(), tile_array![0s, 0s, 0s]);
        assert_eq!(*kezi_005s.tiles(), tile_array![0s, 0s, 5s]);
        assert_eq!(*kezi_050s.tiles(), tile_array![0s, 0s, 5s]);
        assert_eq!(*kezi_055s.tiles(), tile_array![0s, 5s, 5s]);
        assert_eq!(*kezi_500s.tiles(), tile_array![5s, 0s, 0s]);
        assert_eq!(*kezi_505s.tiles(), tile_array![5s, 0s, 5s]);
        assert_eq!(*kezi_550s.tiles(), tile_array![5s, 0s, 5s]);
        assert_eq!(*kezi_555s.tiles(), tile_array![5s, 5s, 5s]);

        assert_eq!(kezi_005s.tiles(), kezi_050s.tiles());
        assert_eq!(kezi_505s.tiles(), kezi_550s.tiles());
    }

    #[test]
    fn test_invalid_ming_kezi() {
        _ = MingKezi::new(tile!(1m), tile_array![1m, 2m], Tajia::Xiajia).unwrap_err();
        _ = MingKezi::new(tile!(1m), tile_array![2m, 1m], Tajia::Xiajia).unwrap_err();
        _ = MingKezi::new(tile!(1m), tile_array![2m, 2m], Tajia::Xiajia).unwrap_err();

        let kezi_left = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Shangjia).unwrap();
        let kezi_across = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Duimian).unwrap();
        let kezi_right = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Xiajia).unwrap();

        assert_ne!(*kezi_left.discarder(), *kezi_across.discarder());
        assert_ne!(*kezi_left.discarder(), *kezi_right.discarder());
        assert_ne!(*kezi_across.discarder(), *kezi_right.discarder());
    }
}
