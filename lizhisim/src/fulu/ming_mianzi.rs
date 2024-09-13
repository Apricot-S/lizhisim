pub use super::mianzi::Mianzi;
use super::tajia::Tajia;
use crate::tile::Tile;

pub trait MingMianzi<const N: usize>: Mianzi<N> {
    fn claimed_tile(&self) -> &Tile;
    fn discarder(&self) -> &Tajia;
}
