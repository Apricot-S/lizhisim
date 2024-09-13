use crate::fulu::Tajia;
use crate::tile::Tile;
use anyhow::{bail, ensure, Result};
use arrayvec::ArrayVec;
use hashbrown::HashSet;

const MAX_NUM_HE: usize = 27;

pub struct Fushipai {
    pub tile: Tile,
    pub is_moqie: bool,
    pub is_lizhi_declaration: bool,
    pub claimer: Option<Tajia>,
}

pub struct He {
    tiles: ArrayVec<Fushipai, MAX_NUM_HE>,
    zhenting_tiles: HashSet<Tile>,
}

impl He {
    pub fn new() -> Self {
        Self {
            tiles: ArrayVec::new(),
            zhenting_tiles: HashSet::with_capacity(MAX_NUM_HE),
        }
    }

    #[inline]
    pub fn tiles(&self) -> &[Fushipai] {
        &self.tiles
    }

    pub fn dapai(&mut self, tile: Tile, is_moqie: bool, is_lizhi_declaration: bool) -> Result<()> {
        self.tiles.push(Fushipai {
            tile,
            is_moqie,
            is_lizhi_declaration,
            claimer: None,
        });

        let zhenting_tile = tile.normalize_hongbaopai();
        self.zhenting_tiles.insert(zhenting_tile);
        Ok(())
    }

    pub fn fulu(&mut self, tile: Tile, claimer: Tajia) -> Result<()> {
        let last_tile = match self.tiles.last_mut() {
            Some(tile) => tile,
            None => bail!("River is empty."),
        };
        ensure!(
            last_tile.tile == tile,
            "Tile to be claimed does not match River tile."
        );

        last_tile.claimer = Some(claimer);
        Ok(())
    }

    pub fn find(&self, tile: &Tile) -> bool {
        self.zhenting_tiles.contains(&tile.normalize_hongbaopai())
    }
}
