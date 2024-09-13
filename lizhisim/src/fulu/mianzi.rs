use crate::tile::Tile;

pub trait Mianzi<const N: usize> {
    fn tiles(&self) -> &[Tile; N];
}
