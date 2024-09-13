use super::ming_mianzi::{Mianzi, MingMianzi};
use super::tajia::Tajia;
use crate::tile::Tile;
use anyhow::{ensure, Result};

#[derive(Debug, Clone)]
pub struct DamingGangzi {
    tiles: [Tile; 4],
    discarder: Tajia,
}

impl DamingGangzi {
    pub fn new(claimed_tile: Tile, dazi: [Tile; 3], discarder: Tajia) -> Result<Self> {
        let normalized_claimed_tile = claimed_tile.normalize_hongbaopai();
        ensure!(
            !dazi
                .iter()
                .any(|&t| t.normalize_hongbaopai() != normalized_claimed_tile),
            "Tiles do not form a valid Open Kan: {:?}, {:?}, {:?}, {:?}",
            claimed_tile,
            dazi[0],
            dazi[1],
            dazi[2],
        );

        let mut sorted_dazi = dazi;
        if sorted_dazi.iter().min().unwrap().is_hongbaopai() {
            sorted_dazi.sort();
        }

        Ok(Self {
            tiles: [claimed_tile, sorted_dazi[0], sorted_dazi[1], sorted_dazi[2]],
            discarder,
        })
    }
}

impl Mianzi<4> for DamingGangzi {
    #[inline]
    #[must_use]
    fn tiles(&self) -> &[Tile; 4] {
        &self.tiles
    }
}

impl MingMianzi<4> for DamingGangzi {
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
    fn test_valid_daming_gangzi() {
        let gangzi_1m =
            DamingGangzi::new(tile!(1m), tile_array![1m, 1m, 1m], Tajia::Xiajia).unwrap();
        let gangzi_1p =
            DamingGangzi::new(tile!(1p), tile_array![1p, 1p, 1p], Tajia::Xiajia).unwrap();
        let gangzi_1s =
            DamingGangzi::new(tile!(1s), tile_array![1s, 1s, 1s], Tajia::Xiajia).unwrap();
        let gangzi_1z =
            DamingGangzi::new(tile!(1z), tile_array![1z, 1z, 1z], Tajia::Xiajia).unwrap();

        assert_eq!(*gangzi_1m.tiles(), tile_array![1m, 1m, 1m, 1m]);
        assert_eq!(*gangzi_1p.tiles(), tile_array![1p, 1p, 1p, 1p]);
        assert_eq!(*gangzi_1s.tiles(), tile_array![1s, 1s, 1s, 1s]);
        assert_eq!(*gangzi_1z.tiles(), tile_array![1z, 1z, 1z, 1z]);
    }

    #[test]
    fn test_valid_daming_gangzi_5() {
        let gangzi_0000m =
            DamingGangzi::new(tile!(0m), tile_array![0m, 0m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_0005m =
            DamingGangzi::new(tile!(0m), tile_array![0m, 0m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_0050m =
            DamingGangzi::new(tile!(0m), tile_array![0m, 5m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_0055m =
            DamingGangzi::new(tile!(0m), tile_array![0m, 5m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_0500m =
            DamingGangzi::new(tile!(0m), tile_array![5m, 0m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_0505m =
            DamingGangzi::new(tile!(0m), tile_array![5m, 0m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_0550m =
            DamingGangzi::new(tile!(0m), tile_array![5m, 5m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_0555m =
            DamingGangzi::new(tile!(0m), tile_array![5m, 5m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_5000m =
            DamingGangzi::new(tile!(5m), tile_array![0m, 0m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_5005m =
            DamingGangzi::new(tile!(5m), tile_array![0m, 0m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_5050m =
            DamingGangzi::new(tile!(5m), tile_array![0m, 5m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_5055m =
            DamingGangzi::new(tile!(5m), tile_array![0m, 5m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_5500m =
            DamingGangzi::new(tile!(5m), tile_array![5m, 0m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_5505m =
            DamingGangzi::new(tile!(5m), tile_array![5m, 0m, 5m], Tajia::Xiajia).unwrap();
        let gangzi_5550m =
            DamingGangzi::new(tile!(5m), tile_array![5m, 5m, 0m], Tajia::Xiajia).unwrap();
        let gangzi_5555m =
            DamingGangzi::new(tile!(5m), tile_array![5m, 5m, 5m], Tajia::Xiajia).unwrap();

        assert_eq!(*gangzi_0000m.tiles(), tile_array![0m, 0m, 0m, 0m]);
        assert_eq!(*gangzi_0005m.tiles(), tile_array![0m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_0050m.tiles(), tile_array![0m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_0055m.tiles(), tile_array![0m, 0m, 5m, 5m]);
        assert_eq!(*gangzi_0500m.tiles(), tile_array![0m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_0505m.tiles(), tile_array![0m, 0m, 5m, 5m]);
        assert_eq!(*gangzi_0550m.tiles(), tile_array![0m, 0m, 5m, 5m]);
        assert_eq!(*gangzi_0555m.tiles(), tile_array![0m, 5m, 5m, 5m]);
        assert_eq!(*gangzi_5000m.tiles(), tile_array![5m, 0m, 0m, 0m]);
        assert_eq!(*gangzi_5005m.tiles(), tile_array![5m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_5050m.tiles(), tile_array![5m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_5055m.tiles(), tile_array![5m, 0m, 5m, 5m]);
        assert_eq!(*gangzi_5500m.tiles(), tile_array![5m, 0m, 0m, 5m]);
        assert_eq!(*gangzi_5505m.tiles(), tile_array![5m, 0m, 5m, 5m]);
        assert_eq!(*gangzi_5550m.tiles(), tile_array![5m, 0m, 5m, 5m]);
        assert_eq!(*gangzi_5555m.tiles(), tile_array![5m, 5m, 5m, 5m]);

        assert_eq!(*gangzi_0005m.tiles(), *gangzi_0050m.tiles());
        assert_eq!(*gangzi_0005m.tiles(), *gangzi_0500m.tiles());
        assert_eq!(*gangzi_0055m.tiles(), *gangzi_0550m.tiles());
        assert_eq!(*gangzi_5005m.tiles(), *gangzi_5050m.tiles());
        assert_eq!(*gangzi_5005m.tiles(), *gangzi_5500m.tiles());
        assert_eq!(*gangzi_5055m.tiles(), *gangzi_5550m.tiles());

        let gangzi_0000p =
            DamingGangzi::new(tile!(0p), tile_array![0p, 0p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_0005p =
            DamingGangzi::new(tile!(0p), tile_array![0p, 0p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_0050p =
            DamingGangzi::new(tile!(0p), tile_array![0p, 5p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_0055p =
            DamingGangzi::new(tile!(0p), tile_array![0p, 5p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_0500p =
            DamingGangzi::new(tile!(0p), tile_array![5p, 0p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_0505p =
            DamingGangzi::new(tile!(0p), tile_array![5p, 0p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_0550p =
            DamingGangzi::new(tile!(0p), tile_array![5p, 5p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_0555p =
            DamingGangzi::new(tile!(0p), tile_array![5p, 5p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_5000p =
            DamingGangzi::new(tile!(5p), tile_array![0p, 0p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_5005p =
            DamingGangzi::new(tile!(5p), tile_array![0p, 0p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_5050p =
            DamingGangzi::new(tile!(5p), tile_array![0p, 5p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_5055p =
            DamingGangzi::new(tile!(5p), tile_array![0p, 5p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_5500p =
            DamingGangzi::new(tile!(5p), tile_array![5p, 0p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_5505p =
            DamingGangzi::new(tile!(5p), tile_array![5p, 0p, 5p], Tajia::Xiajia).unwrap();
        let gangzi_5550p =
            DamingGangzi::new(tile!(5p), tile_array![5p, 5p, 0p], Tajia::Xiajia).unwrap();
        let gangzi_5555p =
            DamingGangzi::new(tile!(5p), tile_array![5p, 5p, 5p], Tajia::Xiajia).unwrap();

        assert_eq!(*gangzi_0000p.tiles(), tile_array![0p, 0p, 0p, 0p]);
        assert_eq!(*gangzi_0005p.tiles(), tile_array![0p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_0050p.tiles(), tile_array![0p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_0055p.tiles(), tile_array![0p, 0p, 5p, 5p]);
        assert_eq!(*gangzi_0500p.tiles(), tile_array![0p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_0505p.tiles(), tile_array![0p, 0p, 5p, 5p]);
        assert_eq!(*gangzi_0550p.tiles(), tile_array![0p, 0p, 5p, 5p]);
        assert_eq!(*gangzi_0555p.tiles(), tile_array![0p, 5p, 5p, 5p]);
        assert_eq!(*gangzi_5000p.tiles(), tile_array![5p, 0p, 0p, 0p]);
        assert_eq!(*gangzi_5005p.tiles(), tile_array![5p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_5050p.tiles(), tile_array![5p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_5055p.tiles(), tile_array![5p, 0p, 5p, 5p]);
        assert_eq!(*gangzi_5500p.tiles(), tile_array![5p, 0p, 0p, 5p]);
        assert_eq!(*gangzi_5505p.tiles(), tile_array![5p, 0p, 5p, 5p]);
        assert_eq!(*gangzi_5550p.tiles(), tile_array![5p, 0p, 5p, 5p]);
        assert_eq!(*gangzi_5555p.tiles(), tile_array![5p, 5p, 5p, 5p]);

        assert_eq!(*gangzi_0005p.tiles(), *gangzi_0050p.tiles());
        assert_eq!(*gangzi_0005p.tiles(), *gangzi_0500p.tiles());
        assert_eq!(*gangzi_0055p.tiles(), *gangzi_0550p.tiles());
        assert_eq!(*gangzi_5005p.tiles(), *gangzi_5050p.tiles());
        assert_eq!(*gangzi_5005p.tiles(), *gangzi_5500p.tiles());
        assert_eq!(*gangzi_5055p.tiles(), *gangzi_5550p.tiles());

        let gangzi_0000s =
            DamingGangzi::new(tile!(0s), tile_array![0s, 0s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_0005s =
            DamingGangzi::new(tile!(0s), tile_array![0s, 0s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_0050s =
            DamingGangzi::new(tile!(0s), tile_array![0s, 5s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_0055s =
            DamingGangzi::new(tile!(0s), tile_array![0s, 5s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_0500s =
            DamingGangzi::new(tile!(0s), tile_array![5s, 0s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_0505s =
            DamingGangzi::new(tile!(0s), tile_array![5s, 0s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_0550s =
            DamingGangzi::new(tile!(0s), tile_array![5s, 5s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_0555s =
            DamingGangzi::new(tile!(0s), tile_array![5s, 5s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_5000s =
            DamingGangzi::new(tile!(5s), tile_array![0s, 0s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_5005s =
            DamingGangzi::new(tile!(5s), tile_array![0s, 0s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_5050s =
            DamingGangzi::new(tile!(5s), tile_array![0s, 5s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_5055s =
            DamingGangzi::new(tile!(5s), tile_array![0s, 5s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_5500s =
            DamingGangzi::new(tile!(5s), tile_array![5s, 0s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_5505s =
            DamingGangzi::new(tile!(5s), tile_array![5s, 0s, 5s], Tajia::Xiajia).unwrap();
        let gangzi_5550s =
            DamingGangzi::new(tile!(5s), tile_array![5s, 5s, 0s], Tajia::Xiajia).unwrap();
        let gangzi_5555s =
            DamingGangzi::new(tile!(5s), tile_array![5s, 5s, 5s], Tajia::Xiajia).unwrap();

        assert_eq!(*gangzi_0000s.tiles(), tile_array![0s, 0s, 0s, 0s]);
        assert_eq!(*gangzi_0005s.tiles(), tile_array![0s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_0050s.tiles(), tile_array![0s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_0055s.tiles(), tile_array![0s, 0s, 5s, 5s]);
        assert_eq!(*gangzi_0500s.tiles(), tile_array![0s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_0505s.tiles(), tile_array![0s, 0s, 5s, 5s]);
        assert_eq!(*gangzi_0550s.tiles(), tile_array![0s, 0s, 5s, 5s]);
        assert_eq!(*gangzi_0555s.tiles(), tile_array![0s, 5s, 5s, 5s]);
        assert_eq!(*gangzi_5000s.tiles(), tile_array![5s, 0s, 0s, 0s]);
        assert_eq!(*gangzi_5005s.tiles(), tile_array![5s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_5050s.tiles(), tile_array![5s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_5055s.tiles(), tile_array![5s, 0s, 5s, 5s]);
        assert_eq!(*gangzi_5500s.tiles(), tile_array![5s, 0s, 0s, 5s]);
        assert_eq!(*gangzi_5505s.tiles(), tile_array![5s, 0s, 5s, 5s]);
        assert_eq!(*gangzi_5550s.tiles(), tile_array![5s, 0s, 5s, 5s]);
        assert_eq!(*gangzi_5555s.tiles(), tile_array![5s, 5s, 5s, 5s]);

        assert_eq!(*gangzi_0005s.tiles(), *gangzi_0050s.tiles());
        assert_eq!(*gangzi_0005s.tiles(), *gangzi_0500s.tiles());
        assert_eq!(*gangzi_0055s.tiles(), *gangzi_0550s.tiles());
        assert_eq!(*gangzi_5005s.tiles(), *gangzi_5050s.tiles());
        assert_eq!(*gangzi_5005s.tiles(), *gangzi_5500s.tiles());
        assert_eq!(*gangzi_5055s.tiles(), *gangzi_5550s.tiles());
    }

    #[test]
    fn test_invalid_daming_gangzi() {
        _ = DamingGangzi::new(tile!(1m), tile_array![1m, 1m, 2m], Tajia::Xiajia).unwrap_err();
        _ = DamingGangzi::new(tile!(1m), tile_array![1m, 2m, 1m], Tajia::Xiajia).unwrap_err();
        _ = DamingGangzi::new(tile!(1m), tile_array![1m, 2m, 2m], Tajia::Xiajia).unwrap_err();
        _ = DamingGangzi::new(tile!(1m), tile_array![2m, 1m, 1m], Tajia::Xiajia).unwrap_err();
        _ = DamingGangzi::new(tile!(1m), tile_array![2m, 1m, 2m], Tajia::Xiajia).unwrap_err();
        _ = DamingGangzi::new(tile!(1m), tile_array![2m, 2m, 1m], Tajia::Xiajia).unwrap_err();
        _ = DamingGangzi::new(tile!(1m), tile_array![2m, 2m, 2m], Tajia::Xiajia).unwrap_err();

        let gangzi_left =
            DamingGangzi::new(tile!(1m), tile_array![1m, 1m, 1m], Tajia::Shangjia).unwrap();
        let gangzi_across =
            DamingGangzi::new(tile!(1m), tile_array![1m, 1m, 1m], Tajia::Duimian).unwrap();
        let gangzi_right =
            DamingGangzi::new(tile!(1m), tile_array![1m, 1m, 1m], Tajia::Xiajia).unwrap();

        assert_ne!(*gangzi_left.discarder(), *gangzi_across.discarder());
        assert_ne!(*gangzi_left.discarder(), *gangzi_right.discarder());
        assert_ne!(*gangzi_across.discarder(), *gangzi_right.discarder());
    }
}
