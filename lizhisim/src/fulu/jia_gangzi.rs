use super::ming_kezi::MingKezi;
use super::ming_mianzi::{Mianzi, MingMianzi};
use super::tajia::Tajia;
use crate::tile::Tile;
use anyhow::{ensure, Result};

#[derive(Debug, Clone)]
pub struct JiaGangzi {
    tiles: [Tile; 4],
    discarder: Tajia,
}

impl JiaGangzi {
    pub fn new(added_tile: Tile, kezi: MingKezi) -> Result<Self> {
        let normalized_added_tile = added_tile.normalize_hongbaopai();
        ensure!(
            !kezi
                .tiles()
                .iter()
                .any(|&t| t.normalize_hongbaopai() != normalized_added_tile),
            "Tiles do not form a valid Added Kan: {:?}, {:?}, {:?}, {:?}",
            kezi.tiles()[0],
            kezi.tiles()[1],
            kezi.tiles()[2],
            added_tile,
        );

        Ok(Self {
            tiles: [
                kezi.tiles()[0],
                kezi.tiles()[1],
                kezi.tiles()[2],
                added_tile,
            ],
            discarder: kezi.discarder().clone(),
        })
    }

    #[inline]
    #[must_use]
    pub fn added_tile(&self) -> &Tile {
        &self.tiles[3]
    }

    #[inline]
    #[must_use]
    pub fn kezi(&self) -> &[Tile] {
        &self.tiles[0..3]
    }
}

impl Mianzi<4> for JiaGangzi {
    #[inline]
    #[must_use]
    fn tiles(&self) -> &[Tile; 4] {
        &self.tiles
    }
}

impl MingMianzi<4> for JiaGangzi {
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
    fn test_valid_jia_gangzi() {
        let kezi_1m = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Xiajia).unwrap();
        let kezi_1p = MingKezi::new(tile!(1p), tile_array![1p, 1p], Tajia::Xiajia).unwrap();
        let kezi_1s = MingKezi::new(tile!(1s), tile_array![1s, 1s], Tajia::Xiajia).unwrap();
        let kezi_1z = MingKezi::new(tile!(1z), tile_array![1z, 1z], Tajia::Xiajia).unwrap();

        let gangzi_1m = JiaGangzi::new(tile!(1m), kezi_1m).unwrap();
        let gangzi_1p = JiaGangzi::new(tile!(1p), kezi_1p).unwrap();
        let gangzi_1s = JiaGangzi::new(tile!(1s), kezi_1s).unwrap();
        let gangzi_1z = JiaGangzi::new(tile!(1z), kezi_1z).unwrap();

        assert_eq!(*gangzi_1m.tiles(), tile_array![1m, 1m, 1m, 1m]);
        assert_eq!(*gangzi_1p.tiles(), tile_array![1p, 1p, 1p, 1p]);
        assert_eq!(*gangzi_1s.tiles(), tile_array![1s, 1s, 1s, 1s]);
        assert_eq!(*gangzi_1z.tiles(), tile_array![1z, 1z, 1z, 1z]);
    }

    #[test]
    fn test_valid_jia_gangzi_5() {
        let kezi_000m = MingKezi::new(tile!(0m), tile_array![0m, 0m], Tajia::Xiajia).unwrap();
        let kezi_000p = MingKezi::new(tile!(0p), tile_array![0p, 0p], Tajia::Xiajia).unwrap();
        let kezi_000s = MingKezi::new(tile!(0s), tile_array![0s, 0s], Tajia::Xiajia).unwrap();
        let kezi_555m = MingKezi::new(tile!(5m), tile_array![5m, 5m], Tajia::Xiajia).unwrap();
        let kezi_555p = MingKezi::new(tile!(5p), tile_array![5p, 5p], Tajia::Xiajia).unwrap();
        let kezi_555s = MingKezi::new(tile!(5s), tile_array![5s, 5s], Tajia::Xiajia).unwrap();

        let gangzi_0005m = JiaGangzi::new(tile!(5m), kezi_000m).unwrap();
        let gangzi_0005p = JiaGangzi::new(tile!(5p), kezi_000p).unwrap();
        let gangzi_0005s = JiaGangzi::new(tile!(5s), kezi_000s).unwrap();
        let gangzi_5550m = JiaGangzi::new(tile!(0m), kezi_555m).unwrap();
        let gangzi_5550p = JiaGangzi::new(tile!(0p), kezi_555p).unwrap();
        let gangzi_5550s = JiaGangzi::new(tile!(0s), kezi_555s).unwrap();

        assert_eq!(*gangzi_0005m.tiles(), tile_array![0m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_0005p.tiles(), tile_array![0p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_0005s.tiles(), tile_array![0s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_5550m.tiles(), tile_array![5m, 5m, 5m, 0m]);
        assert_eq!(*gangzi_5550p.tiles(), tile_array![5p, 5p, 5p, 0p]);
        assert_eq!(*gangzi_5550s.tiles(), tile_array![5s, 5s, 5s, 0s]);
    }

    #[test]
    fn test_invalid_jia_gangzi() {
        let kezi_1m = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Xiajia).unwrap();

        _ = JiaGangzi::new(tile!(2m), kezi_1m).unwrap_err();

        let kezi_left = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Shangjia).unwrap();
        let kezi_across = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Duimian).unwrap();
        let kezi_right = MingKezi::new(tile!(1m), tile_array![1m, 1m], Tajia::Xiajia).unwrap();

        let gangzi_left = JiaGangzi::new(tile!(1m), kezi_left).unwrap();
        let gangzi_across = JiaGangzi::new(tile!(1m), kezi_across).unwrap();
        let gangzi_right = JiaGangzi::new(tile!(1m), kezi_right).unwrap();

        assert_ne!(*gangzi_left.discarder(), *gangzi_across.discarder());
        assert_ne!(*gangzi_left.discarder(), *gangzi_right.discarder());
        assert_ne!(*gangzi_across.discarder(), *gangzi_right.discarder());
    }
}
