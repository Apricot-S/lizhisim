use crate::fulu::{
    AnGangzi, ClaimedTilePosition, DamingGangzi, FuluMianzi, JiaGangzi, Mianzi, MingKezi,
    MingMianzi, MingShunzi, Tajia,
};
use crate::tile;
use crate::tile::{Tile, NUM_SAME_TILE, NUM_SHUPAI_RANK, NUM_TILE_INDEX};
use crate::{matches_tile_index, tile_index};
use anyhow::{bail, Result};
use arrayvec::ArrayVec;

const MAX_NUM_BINGPAI: usize = 14;
const MAX_NUM_FULU: usize = 4;
const MAX_NUM_DAPAI_CANDIDATES: usize = MAX_NUM_BINGPAI;
const MAX_NUM_CHI_CANDIDATES: usize = 5; // 4 [2, 3], [3, r5], [3, 5], [r5, 6], [5, 6]
const MAX_NUM_PENG_CANDIDATES: usize = 2; // 5 [r5, 5], [5, 5]
const MAX_NUM_DAMINGGANG_CANDIDATES: usize = 1;
const MAX_NUM_ANGANG_JIAGANG_CANDIDATES: usize = 3; // 1234 [111], [222], [333]

#[derive(Clone)]
enum Zimopai<'a> {
    Some(Tile),
    Unknown,
    AfterChi(&'a MingShunzi),
    AfterPeng(&'a MingKezi),
    None,
}

#[derive(Clone)]
pub struct Shoupai<'a> {
    bingpai: [u8; NUM_TILE_INDEX],
    num_hidden_bingpai: u8,
    fulu_list: ArrayVec<FuluMianzi, MAX_NUM_FULU>,
    num_ba_baopai: u8,
    zimopai: Zimopai<'a>,
    is_lizhi: bool,
}

impl<'a> Shoupai<'a> {
    #[must_use]
    pub fn new(qipai: Vec<Option<Tile>>) -> Result<Self> {
        let mut bingpai = [0u8; NUM_TILE_INDEX];
        let mut num_hidden_bingpai = 0u8;

        for tile in qipai {
            match tile {
                None => num_hidden_bingpai += 1,
                Some(open) => {
                    bingpai[open.to_usize()] += 1;
                }
            }
        }

        if let Some(index) = bingpai.iter().position(|&n| n > NUM_SAME_TILE) {
            bail!(
                "There are 5 or more same tiles.: {}",
                Tile::try_from(index as u8).unwrap()
            );
        }
        if (bingpai[tile_index!(5m) as usize] + bingpai[tile_index!(0m) as usize]) > NUM_SAME_TILE {
            bail!("There are a total of 5 or more 5m and Red 5m.");
        }
        if (bingpai[tile_index!(5p) as usize] + bingpai[tile_index!(0p) as usize]) > NUM_SAME_TILE {
            bail!("There are a total of 5 or more 5p and Red 5p.");
        }
        if (bingpai[tile_index!(5s) as usize] + bingpai[tile_index!(0s) as usize]) > NUM_SAME_TILE {
            bail!("There are a total of 5 or more 5s and Red 5s.");
        }

        Ok(Self {
            bingpai,
            num_hidden_bingpai,
            fulu_list: ArrayVec::new(),
            num_ba_baopai: 0,
            zimopai: Zimopai::None,
            is_lizhi: false,
        })
    }

    pub fn zimo(&mut self, zimopai: Option<Tile>) -> Result<()> {
        let Zimopai::None = self.zimopai else {
            bail!("Tsumo was attempted while already holding a Tsumo tile.");
        };

        match zimopai {
            None => {
                self.num_hidden_bingpai += 1;
                self.zimopai = Zimopai::Unknown;
            }
            Some(tile) => {
                let index = tile.to_usize();
                if self.bingpai[index] >= NUM_SAME_TILE {
                    bail!("There are 5 or more same tiles.: {}", tile);
                }

                if matches_tile_index!(*tile.as_u8(), 5m | 0m) {
                    if (self.bingpai[tile_index!(5m) as usize]
                        + self.bingpai[tile_index!(0m) as usize])
                        > NUM_SAME_TILE
                    {
                        bail!("There are a total of 5 or more 5m and Red 5m.: {}", tile);
                    }
                }
                if matches_tile_index!(*tile.as_u8(), 5p | 0p) {
                    if (self.bingpai[tile_index!(5p) as usize]
                        + self.bingpai[tile_index!(0p) as usize])
                        > NUM_SAME_TILE
                    {
                        bail!("There are a total of 5 or more 5p and Red 5p.: {}", tile);
                    }
                }
                if matches_tile_index!(*tile.as_u8(), 5s | 0s) {
                    if (self.bingpai[tile_index!(5s) as usize]
                        + self.bingpai[tile_index!(0s) as usize])
                        > NUM_SAME_TILE
                    {
                        bail!("There are a total of 5 or more 5s and Red 5s.: {}", tile);
                    }
                }

                self.bingpai[index] += 1;
                self.zimopai = Zimopai::Some(tile);
            }
        }

        Ok(())
    }

    #[inline]
    fn decrease(&mut self, tile: &Tile) -> Result<()> {
        let count = &mut self.bingpai[tile.to_usize()];
        if *count == 0 {
            if self.num_hidden_bingpai == 0 {
                bail!("The specified tile is not left in the hand.");
            }
            self.num_hidden_bingpai -= 1;
        } else {
            *count -= 1;
        }
        Ok(())
    }

    pub fn dapai(&mut self, tile: Tile, is_moqie: bool, is_lizhi_declaration: bool) -> Result<()> {
        match &self.zimopai {
            Zimopai::Unknown | Zimopai::None => {
                bail!("Discarding a tile was attempted when there were no Tsumo tiles yet.");
            }
            Zimopai::AfterChi(_) | Zimopai::AfterPeng(_) => {}
            Zimopai::Some(zimopai) => {
                if is_moqie && (*zimopai != tile) {
                    bail!(
                        "The tile to be Moqie and the Tsumo tile do not match.\n\
                        Moqie: {} Tsumo: {}",
                        tile,
                        zimopai
                    );
                }
            }
        }

        self.decrease(&tile)?;
        self.zimopai = Zimopai::None;
        if is_lizhi_declaration {
            self.is_lizhi = true;
        }
        Ok(())
    }

    pub fn chi(&'a mut self, shunzi: MingShunzi) -> Result<()> {
        let Zimopai::None = self.zimopai else {
            bail!("Chii was attempted in a state where Claiming was not possible.");
        };

        shunzi.tiles().iter().try_for_each(|t| self.decrease(t))?;

        self.fulu_list.push(FuluMianzi::MingShunzi(shunzi));
        if let Some(FuluMianzi::MingShunzi(s)) = self.fulu_list.last() {
            self.zimopai = Zimopai::AfterChi(&s);
        } else {
            bail!("Internal Error: Could not get reference to last meld.");
        };
        Ok(())
    }

    pub fn peng(&'a mut self, kezi: MingKezi) -> Result<()> {
        let Zimopai::None = self.zimopai else {
            bail!("Pon was attempted in a state where Claiming was not possible.");
        };

        kezi.tiles().iter().try_for_each(|t| self.decrease(t))?;

        self.fulu_list.push(FuluMianzi::MingKezi(kezi));
        if let Some(FuluMianzi::MingKezi(k)) = self.fulu_list.last() {
            self.zimopai = Zimopai::AfterPeng(&k);
        } else {
            bail!("Internal Error: Could not get reference to last meld.");
        };
        Ok(())
    }

    pub fn daminggang(&mut self, gangzi: DamingGangzi) -> Result<()> {
        let Zimopai::None = self.zimopai else {
            bail!("Open Kan was attempted in a state where Claiming was not possible.");
        };

        gangzi.tiles().iter().try_for_each(|t| self.decrease(t))?;

        self.fulu_list.push(FuluMianzi::DamingGangzi(gangzi));
        Ok(())
    }

    pub fn angang(&'a mut self, gangzi: AnGangzi) -> Result<()> {
        if let Zimopai::None | Zimopai::AfterChi(_) | Zimopai::AfterPeng(_) = &self.zimopai {
            bail!("It is not possible to perform Concealed Kan except after Tsumo.");
        }

        gangzi.tiles().iter().try_for_each(|t| self.decrease(t))?;

        self.fulu_list.push(FuluMianzi::AnGangzi(gangzi));

        self.zimopai = Zimopai::None;
        Ok(())
    }

    pub fn jiagang(&'a mut self, gangzi: JiaGangzi) -> Result<()> {
        if let Zimopai::None | Zimopai::AfterChi(_) | Zimopai::AfterPeng(_) = &self.zimopai {
            bail!("It is not possible to perform Added Kan except after Tsumo.");
        }

        let g_kezi = gangzi.kezi();
        let matching_kezi_index = self
            .fulu_list
            .iter()
            .position(|m| matches!(m, FuluMianzi::MingKezi(kezi) if *kezi.tiles() == g_kezi));

        match matching_kezi_index {
            Some(i) => {
                self.decrease(gangzi.added_tile())?;
                self.fulu_list[i] = FuluMianzi::JiaGangzi(gangzi);
            }
            None => bail!("There was no Melded Triplets to fit the Added Kan."),
        };

        self.zimopai = Zimopai::None;
        Ok(())
    }

    pub fn babei(&mut self) -> Result<()> {
        if let Zimopai::None | Zimopai::AfterChi(_) | Zimopai::AfterPeng(_) = &self.zimopai {
            bail!("It is not possible to perform Kita except after Tsumo.");
        }

        self.decrease(&tile!(4z))?;
        self.num_ba_baopai += 1;
        self.zimopai = Zimopai::None;
        Ok(())
    }

    #[must_use]
    pub fn is_menqian(&self) -> bool {
        !self
            .fulu_list
            .iter()
            .any(|f| !matches!(f, FuluMianzi::AnGangzi(_)))
    }

    #[inline]
    #[must_use]
    pub fn is_lizhi(&self) -> bool {
        self.is_lizhi
    }

    #[must_use]
    pub fn get_dapai_candidates(
        &self,
        check_shiti: bool,
    ) -> Option<ArrayVec<Tile, MAX_NUM_DAPAI_CANDIDATES>> {
        if let Zimopai::None = &self.zimopai {
            return None;
        }

        let mut candidates = ArrayVec::<Tile, MAX_NUM_DAPAI_CANDIDATES>::new();

        if self.is_lizhi() {
            match &self.zimopai {
                Zimopai::Some(t) => {
                    candidates.push(*t);
                    return Some(candidates);
                }
                _ => panic!("The state of the Hand during Riichi is invalid."),
            }
        }

        let mut deny = ArrayVec::<usize, 3>::new(); // Includes Suji, Red Dora
        if check_shiti {
            match &self.zimopai {
                Zimopai::AfterChi(s) => {
                    let t = s.claimed_tile();
                    if matches_tile_index!(t.as_u8(), 5m | 5p | 5s | 0m | 0p | 0s) {
                        deny.push(t.decorate_hongbaopai().to_usize());
                        deny.push(t.normalize_hongbaopai().to_usize());
                    } else {
                        deny.push(t.to_usize());
                    }

                    if let Some(j) = s.shiti_jin() {
                        if matches_tile_index!(j.as_u8(), 5m | 5p | 5s) {
                            deny.push(j.decorate_hongbaopai().to_usize());
                            deny.push(j.normalize_hongbaopai().to_usize());
                        } else {
                            deny.push(j.to_usize());
                        }
                    }
                }
                Zimopai::AfterPeng(k) => {
                    let t = k.claimed_tile();
                    if matches_tile_index!(t.as_u8(), 5m | 5p | 5s | 0m | 0p | 0s) {
                        deny.push(t.decorate_hongbaopai().to_usize());
                        deny.push(t.normalize_hongbaopai().to_usize());
                    } else {
                        deny.push(t.to_usize());
                    }
                }
                _ => {}
            }
        }

        let zimopai_index = if let Zimopai::Some(t) = &self.zimopai {
            t.to_usize()
        } else {
            usize::MAX
        };
        self.bingpai.iter().enumerate().for_each(|(i, count)| {
            if *count == 0 {
                ()
            } else if deny.contains(&i) {
                ()
            } else if (i == zimopai_index) && (*count == 1) {
                // Only a drawn tile can be discarded
                ()
            } else {
                candidates.push(Tile::new_unchecked(i as u8))
            }
        });

        // The drawn tile is added to the end of the list.
        if let Zimopai::Some(t) = &self.zimopai {
            candidates.push(*t);
        }
        Some(candidates)
    }

    #[must_use]
    pub fn get_chi_candidates(
        &self,
        tile: &Tile,
        check_shiti: bool,
    ) -> Option<ArrayVec<MingShunzi, MAX_NUM_CHI_CANDIDATES>> {
        let Zimopai::None = self.zimopai else {
            return None;
        };

        let mut candidates = ArrayVec::<MingShunzi, MAX_NUM_CHI_CANDIDATES>::new();

        if tile.is_zipai() {
            return Some(candidates);
        }
        if self.is_lizhi() {
            return Some(candidates);
        }

        let index = tile.normalize_hongbaopai().to_usize();
        let rank = (index as u8) % NUM_SHUPAI_RANK;
        let num_bingpai: u8 = self.bingpai.iter().sum();

        let can_chi = |jin: Option<usize>| -> bool {
            let mut count: u8 = 0;
            count += self.bingpai[index];
            if index == tile_index!(5m) as usize {
                count += self.bingpai[tile_index!(0m) as usize]
            }
            if index == tile_index!(5p) as usize {
                count += self.bingpai[tile_index!(0p) as usize]
            }
            if index == tile_index!(5s) as usize {
                count += self.bingpai[tile_index!(0s) as usize]
            }

            if let Some(j) = jin {
                count += self.bingpai[j];
                if j == tile_index!(5m) as usize {
                    count += self.bingpai[tile_index!(0m) as usize]
                }
                if j == tile_index!(5p) as usize {
                    count += self.bingpai[tile_index!(0p) as usize]
                }
                if j == tile_index!(5s) as usize {
                    count += self.bingpai[tile_index!(0s) as usize]
                }
            }

            count < (num_bingpai - 2)
        };

        // In case of { claimed_tile: 7x, dazi: [0x, 6x] }
        if rank == 6 && self.bingpai[index - 1] > 0 {
            let tile_1 = tile.prev();
            let tile_0 = tile_1.prev().decorate_hongbaopai();
            if self.bingpai[tile_0.to_usize()] > 0 && !check_shiti || can_chi(Some(index - 3)) {
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // In case of { claimed_tile: 6x, dazi: [4x, 0x] }
        if rank == 5 && self.bingpai[index - 2] > 0 {
            let tile_1 = tile.prev().decorate_hongbaopai();
            if self.bingpai[tile_1.to_usize()] > 0 && !check_shiti || can_chi(Some(index - 3)) {
                let tile_0 = tile_1.prev();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // If the claimed tile position is high.
        if rank >= 2 && self.bingpai[index - 2] > 0 && self.bingpai[index - 1] > 0 {
            if !check_shiti || can_chi(if rank == 2 { None } else { Some(index - 3) }) {
                let tile_1 = tile.prev();
                let tile_0 = tile_1.prev();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // In case of { claimed_tile: 6x, dazi: [0x, 7x] }
        if rank == 5 && self.bingpai[index + 1] > 0 {
            let tile_0 = tile.prev().decorate_hongbaopai();
            if self.bingpai[tile_0.to_usize()] > 0 && !check_shiti || can_chi(None) {
                let tile_1 = tile.next();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // In case of { claimed_tile: 4x, dazi: [3x, 0x] }
        if rank == 3 && self.bingpai[index - 1] > 0 {
            let tile_1 = tile.next().decorate_hongbaopai();
            if self.bingpai[tile_1.to_usize()] > 0 && !check_shiti || can_chi(None) {
                let tile_0 = tile.prev();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // If the claimed tile position is middle.
        if rank >= 1 && rank <= 7 && self.bingpai[index - 1] > 0 && self.bingpai[index + 1] > 0 {
            if !check_shiti || can_chi(None) {
                let tile_0 = tile.prev();
                let tile_1 = tile.next();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // In case of { claimed_tile: 4x, dazi: [0x, 6x] }
        if rank == 3 && self.bingpai[index + 2] > 0 {
            let tile_0 = tile.next().decorate_hongbaopai();
            if self.bingpai[tile_0.to_usize()] > 0 && !check_shiti || can_chi(Some(index + 3)) {
                let tile_1 = tile_0.next();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // In case of { claimed_tile: 3x, dazi: [4x, 0x] }
        if rank == 2 && self.bingpai[index + 1] > 0 {
            let tile_0 = tile.next();
            let tile_1 = tile_0.next().decorate_hongbaopai();
            if self.bingpai[tile_1.to_usize()] > 0 && !check_shiti || can_chi(Some(index + 3)) {
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        // If the claimed tile position is low.
        if rank <= 6 && self.bingpai[index + 1] > 0 && self.bingpai[index + 2] > 0 {
            if !check_shiti || can_chi(if rank == 6 { None } else { Some(index + 3) }) {
                let tile_0 = tile.next();
                let tile_1 = tile_0.next();
                candidates.push(MingShunzi::new(*tile, [tile_0, tile_1]).unwrap())
            }
        }

        Some(candidates)
    }

    #[must_use]
    pub fn get_peng_candidates(
        &self,
        tile: &Tile,
        discarder: &Tajia,
    ) -> Option<ArrayVec<MingKezi, MAX_NUM_PENG_CANDIDATES>> {
        let Zimopai::None = self.zimopai else {
            return None;
        };

        let mut candidates = ArrayVec::<MingKezi, MAX_NUM_PENG_CANDIDATES>::new();

        if self.is_lizhi() {
            return Some(candidates);
        }

        if matches_tile_index!(tile.as_u8(), 5m | 5p | 5s | 0m | 0p | 0s) {
            let red = tile.decorate_hongbaopai();
            let normal = tile.normalize_hongbaopai();
            let num_red = self.bingpai[red.to_usize()];
            let num_normal = self.bingpai[normal.to_usize()];
            if num_red >= 2 {
                candidates.push(MingKezi::new(*tile, [red, red], discarder.clone()).unwrap());
            }
            if (num_red >= 1) && (num_normal >= 1) {
                candidates.push(MingKezi::new(*tile, [red, normal], discarder.clone()).unwrap());
            }
            if num_normal >= 2 {
                candidates.push(MingKezi::new(*tile, [normal, normal], discarder.clone()).unwrap());
            }
        } else {
            if self.bingpai[tile.to_usize()] >= 2 {
                candidates.push(MingKezi::new(*tile, [*tile, *tile], discarder.clone()).unwrap());
            }
        }

        Some(candidates)
    }

    #[must_use]
    pub fn get_daminggang_candidates(
        &self,
        tile: &Tile,
        discarder: &Tajia,
    ) -> Option<ArrayVec<DamingGangzi, MAX_NUM_DAMINGGANG_CANDIDATES>> {
        let Zimopai::None = self.zimopai else {
            return None;
        };

        let mut candidates = ArrayVec::<DamingGangzi, MAX_NUM_DAMINGGANG_CANDIDATES>::new();

        if self.is_lizhi() {
            return Some(candidates);
        };

        if matches_tile_index!(tile.as_u8(), 5m | 5p | 5s | 0m | 0p | 0s) {
            let red = tile.decorate_hongbaopai();
            let normal = tile.normalize_hongbaopai();
            let num_red = self.bingpai[red.to_usize()];
            let num_normal = self.bingpai[normal.to_usize()];
            if num_red == 3 {
                candidates
                    .push(DamingGangzi::new(*tile, [red, red, red], discarder.clone()).unwrap());
            } else if (num_red == 2) && (num_normal == 1) {
                candidates
                    .push(DamingGangzi::new(*tile, [red, red, normal], discarder.clone()).unwrap());
            } else if (num_red == 1) && (num_normal == 2) {
                candidates.push(
                    DamingGangzi::new(*tile, [red, normal, normal], discarder.clone()).unwrap(),
                );
            } else if num_normal == 3 {
                candidates.push(
                    DamingGangzi::new(*tile, [normal, normal, normal], discarder.clone()).unwrap(),
                );
            }
        } else {
            if self.bingpai[tile.to_usize()] == 3 {
                candidates.push(
                    DamingGangzi::new(*tile, [*tile, *tile, *tile], discarder.clone()).unwrap(),
                );
            }
        }

        Some(candidates)
    }
}
