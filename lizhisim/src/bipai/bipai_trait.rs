use crate::tile::Tile;
use anyhow::Result;

pub trait Bipai {
    fn to_baopai(baopai_indicator: &Tile) -> Tile {
        baopai_indicator.next()
    }

    fn left_tile_count(&self) -> Result<u8>;
    fn baopai_indicators(&self) -> Option<&[Tile]>;
    fn li_baopai_indicators(&self) -> Result<Option<&[Tile]>>;

    fn close(&mut self);
    fn zimo(&mut self) -> Result<Tile>;
    fn lingshangzimo(&mut self) -> Result<Tile>;
    fn kaigang(&mut self) -> Result<()>;
}
