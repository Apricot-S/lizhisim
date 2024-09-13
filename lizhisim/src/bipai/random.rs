use super::bipai_trait::Bipai;
use super::constants::{
    BIPAI_RED_5M_INDEX_4, BIPAI_RED_5P_INDEX_3, BIPAI_RED_5P_INDEX_4, BIPAI_RED_5S_INDEX_3,
    BIPAI_RED_5S_INDEX_4, INITIAL_BIPAI_WITHOUT_HONGBAOPAI_3, INITIAL_BIPAI_WITHOUT_HONGBAOPAI_4,
    MAX_GANG_COUNT, MAX_NUM_BAOPAI, NUM_BIPAI_4, NUM_LINGSHANGPAI, NUM_LINGSHANGPAI_WITH_BABEI,
    NUM_WANGPAI,
};
use crate::rule::{NumPlayer, Rule};
use crate::tile;
use crate::tile::Tile;
use anyhow::{bail, ensure, Result};
use arraydeque::ArrayDeque;
use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct RandomBipai {
    closed: bool,
    rule: Rule,
    tiles: ArrayDeque<Tile, NUM_BIPAI_4>,
    baopai_indicators: Option<ArrayVec<Tile, MAX_NUM_BAOPAI>>,
    li_baopai_indicators: Option<ArrayVec<Tile, MAX_NUM_BAOPAI>>,
    num_lingshangpai: usize,
    gang_count: usize,
    should_kaigang: bool,
}

impl RandomBipai {
    #[must_use]
    pub fn new(rule: Rule, mut rng: impl Rng) -> Self {
        let tiles: ArrayDeque<Tile, NUM_BIPAI_4> = match *rule.num_player() {
            NumPlayer::Four => {
                let mut init_tiles = INITIAL_BIPAI_WITHOUT_HONGBAOPAI_4.clone();

                let num_r5m = rule.hongbaopai_count().wanzi as usize;
                let num_r5p = rule.hongbaopai_count().bingzi as usize;
                let num_r5s = rule.hongbaopai_count().suozi as usize;
                for i in 0..num_r5m {
                    init_tiles[BIPAI_RED_5M_INDEX_4 + i] = tile!(0m);
                }
                for i in 0..num_r5p {
                    init_tiles[BIPAI_RED_5P_INDEX_4 + i] = tile!(0p);
                }
                for i in 0..num_r5s {
                    init_tiles[BIPAI_RED_5S_INDEX_4 + i] = tile!(0s);
                }

                init_tiles.shuffle(&mut rng);
                let mut shuffled_tiles = ArrayDeque::<Tile, NUM_BIPAI_4>::new();
                shuffled_tiles.extend_back(init_tiles);
                shuffled_tiles
            }
            NumPlayer::Three => {
                let mut init_tiles = INITIAL_BIPAI_WITHOUT_HONGBAOPAI_3.clone();

                let num_r5p = rule.hongbaopai_count().bingzi as usize;
                let num_r5s = rule.hongbaopai_count().suozi as usize;
                for i in 0..num_r5p {
                    init_tiles[BIPAI_RED_5P_INDEX_3 + i] = tile!(0p);
                }
                for i in 0..num_r5s {
                    init_tiles[BIPAI_RED_5S_INDEX_3 + i] = tile!(0s);
                }

                init_tiles.shuffle(&mut rng);
                let mut shuffled_tiles = ArrayDeque::<Tile, NUM_BIPAI_4>::new();
                shuffled_tiles.extend_back(init_tiles);
                shuffled_tiles
            }
        };

        let num_lingshangpai = match *rule.num_player() {
            NumPlayer::Four => NUM_LINGSHANGPAI,
            NumPlayer::Three => {
                if *rule.has_ba_baopai() {
                    NUM_LINGSHANGPAI_WITH_BABEI
                } else {
                    NUM_LINGSHANGPAI
                }
            }
        };

        let baopai_indicators = if *rule.has_biao_baopai() {
            let mut b = ArrayVec::<Tile, MAX_NUM_BAOPAI>::new();
            b.push(tiles[num_lingshangpai]);
            Some(b)
        } else {
            None
        };

        let li_baopai_indicators = if *rule.has_li_baopai() {
            let mut l = ArrayVec::<Tile, MAX_NUM_BAOPAI>::new();
            l.push(tiles[num_lingshangpai + MAX_NUM_BAOPAI]);
            Some(l)
        } else {
            None
        };

        Self {
            closed: false,
            rule,
            tiles,
            baopai_indicators,
            li_baopai_indicators,
            num_lingshangpai,
            gang_count: 0,
            should_kaigang: false,
        }
    }

    #[inline]
    #[must_use]
    fn has_left_tile(&self) -> bool {
        self.tiles.len() > NUM_WANGPAI
    }
}

impl Bipai for RandomBipai {
    #[must_use]
    fn left_tile_count(&self) -> Result<u8> {
        let left_bipai_count = self.tiles.len();
        ensure!(left_bipai_count >= NUM_WANGPAI, "No tiles left.");
        Ok((left_bipai_count - NUM_WANGPAI) as u8)
    }

    #[inline]
    #[must_use]
    fn baopai_indicators(&self) -> Option<&[Tile]> {
        match &self.baopai_indicators {
            None => None,
            Some(b) => Some(b.as_slice()),
        }
    }

    #[inline]
    #[must_use]
    fn li_baopai_indicators(&self) -> Result<Option<&[Tile]>> {
        ensure!(self.closed, "Round is not finished yet.");
        match &self.li_baopai_indicators {
            None => Ok(None),
            Some(l) => Ok(Some(l.as_slice())),
        }
    }

    #[inline]
    fn close(&mut self) {
        self.closed = true;
    }

    #[must_use]
    fn zimo(&mut self) -> Result<Tile> {
        ensure!(!self.closed, "Round has ended.");
        ensure!(self.has_left_tile(), "No tiles left.");
        ensure!(
            !self.should_kaigang,
            "`zimo` was called before executing `kaikang`."
        );

        if let Some(zimopai) = self.tiles.pop_back() {
            Ok(zimopai)
        } else {
            bail!("Failed to `zimo`.")
        }
    }

    #[must_use]
    fn lingshangzimo(&mut self) -> Result<Tile> {
        ensure!(!self.closed, "Round has ended.");
        ensure!(self.has_left_tile(), "No tiles left.");
        ensure!(
            !self.should_kaigang,
            "`lingshangzimo` was called before executing `kaikang`."
        );
        ensure!(
            self.gang_count < MAX_GANG_COUNT,
            "The fifth Kan is not allowed."
        );

        if *self.rule.has_gang_biao_baopai() {
            self.should_kaigang = true
        }

        self.gang_count += 1;

        if let Some(lingshangzimopai) = self.tiles.pop_front() {
            Ok(lingshangzimopai)
        } else {
            bail!("Failed to `lingshangzimo`.")
        }
    }

    fn kaigang(&mut self) -> Result<()> {
        ensure!(!self.closed, "Round has ended.");
        ensure!(
            self.should_kaigang,
            "Current state is not possible to `kaigang`.",
        );

        if *self.rule.has_gang_biao_baopai() {
            if let Some(baopai_indicators) = self.baopai_indicators.as_mut() {
                baopai_indicators.push(self.tiles[self.num_lingshangpai]);
            }
        }

        if *self.rule.has_gang_li_baopai() {
            if let Some(li_baopai_indicators) = self.li_baopai_indicators.as_mut() {
                li_baopai_indicators.push(self.tiles[self.num_lingshangpai + MAX_NUM_BAOPAI]);
            }
        }

        self.should_kaigang = false;
        Ok(())
    }
}
