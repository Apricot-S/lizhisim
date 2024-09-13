use super::ming_mianzi::Mianzi;
use crate::tile::Tile;
use anyhow::{ensure, Result};

#[derive(Debug, Clone)]
pub struct AnGangzi {
    tiles: [Tile; 4],
}

impl AnGangzi {
    #[must_use]
    pub fn new(dazi: [Tile; 4]) -> Result<Self> {
        let normalized_dazi = dazi.map(|t| t.normalize_hongbaopai());
        ensure!(
            !normalized_dazi.iter().any(|&t| t != normalized_dazi[0]),
            "Tiles do not form a valid Concealed Kan: {:?}, {:?}, {:?}, {:?}",
            dazi[0],
            dazi[1],
            dazi[2],
            dazi[3],
        );

        let mut sorted_dazi = dazi;
        if sorted_dazi.iter().min().unwrap().is_hongbaopai() {
            sorted_dazi.sort();
        }

        Ok(Self { tiles: sorted_dazi })
    }
}

impl Mianzi<4> for AnGangzi {
    #[inline]
    #[must_use]
    fn tiles(&self) -> &[Tile; 4] {
        &self.tiles
    }
}
