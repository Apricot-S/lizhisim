// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use core::marker::PhantomData;

use thiserror::Error;

use crate::bingpai::Bingpai;
use crate::seat::FourPlayer;
use crate::tile::TileKind;
use crate::tile_set::TileSet;

const FOUR_PLAYER_QIPAI_TILE_COUNT: usize = 52;
const FOUR_PLAYER_WANGPAI_TILE_COUNT: usize = 14;

mod private {
    pub trait Sealed {}
}

pub trait BipaiSpec: private::Sealed {
    type BipaiTiles: AsRef<[TileKind]>;
}

impl private::Sealed for FourPlayer {}

impl BipaiSpec for FourPlayer {
    type BipaiTiles = [TileKind; 136];
}

#[derive(Debug, Error, PartialEq)]
pub enum BipaiError {
    #[error("bipai tile kind {tile_kind:?} has {actual_count} copies, expected {expected_count}")]
    TileSetMismatch {
        tile_kind: TileKind,
        actual_count: u8,
        expected_count: u8,
    },
    #[error("no tile remains in the live wall")]
    LiveWallExhausted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QipaiPending;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QipaiCompleted;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bipai<P: BipaiSpec, QipaiState = QipaiPending> {
    tiles: P::BipaiTiles,
    tile_set: TileSet,
    remaining_count: usize,
    cursor: usize,
    qipai_state: PhantomData<fn() -> QipaiState>,
}

impl<P: BipaiSpec, QipaiState> Bipai<P, QipaiState> {
    pub fn remaining_count(&self) -> usize {
        self.remaining_count
    }
}

impl Bipai<FourPlayer, QipaiPending> {
    pub fn try_new(tiles: [TileKind; 136], tile_set: TileSet) -> Result<Self, BipaiError> {
        let actual_counts = tiles.iter().fold([0usize; 37], |mut counts, tile_kind| {
            counts[tile_kind.index()] += 1;
            counts
        });

        if let Some((_index, (actual_count, tile_kind))) = TileKind::ALL
            .iter()
            .enumerate()
            .map(|(index, tile_kind)| (index, (actual_counts[index], tile_kind)))
            .find(|(_, (actual_count, tile_kind))| {
                *actual_count != usize::from(tile_set.max_count(**tile_kind))
            })
        {
            return Err(BipaiError::TileSetMismatch {
                tile_kind: *tile_kind,
                actual_count: actual_count as u8,
                expected_count: tile_set.max_count(*tile_kind),
            });
        }

        let remaining_count = tiles.len() - FOUR_PLAYER_WANGPAI_TILE_COUNT;
        Ok(Self {
            tiles,
            tile_set,
            remaining_count,
            cursor: 0,
            qipai_state: PhantomData,
        })
    }

    pub fn qipai(self) -> (Bipai<FourPlayer, QipaiCompleted>, [Bingpai; 4]) {
        let tiles = self.tiles.as_ref();
        let bingpai = core::array::from_fn(|seat_index| {
            let seat_offset = seat_index * 4;
            let mut counts = [0; 37];

            for batch_start in [0, 16, 32] {
                for &tile_kind in &tiles[batch_start + seat_offset..batch_start + seat_offset + 4] {
                    counts[tile_kind.index()] += 1;
                }
            }

            let tile_kind = tiles[48 + seat_index];
            counts[tile_kind.index()] += 1;

            Bingpai::from_validated_counts(counts, self.tile_set.clone())
        });

        (
            Bipai {
                tiles: self.tiles,
                tile_set: self.tile_set,
                remaining_count: self.remaining_count - FOUR_PLAYER_QIPAI_TILE_COUNT,
                cursor: FOUR_PLAYER_QIPAI_TILE_COUNT,
                qipai_state: PhantomData,
            },
            bingpai,
        )
    }
}

impl<P: BipaiSpec> Bipai<P, QipaiCompleted> {
    pub fn zimo(mut self) -> Result<(Self, TileKind), BipaiError> {
        if self.remaining_count() == 0 {
            return Err(BipaiError::LiveWallExhausted);
        }
        let tile_kind = self.tiles.as_ref()[self.cursor];
        self.remaining_count -= 1;
        self.cursor += 1;
        Ok((self, tile_kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn red_three_tiles() -> ([TileKind; 136], TileSet) {
        let tile_set = TileSet::red_three_four_player();

        let mut tiles = [TileKind::M1; 136];
        let mut cursor = 0;
        for tile_kind in TileKind::ALL {
            for _ in 0..tile_set.max_count(tile_kind) {
                tiles[cursor] = tile_kind;
                cursor += 1;
            }
        }
        (tiles, tile_set)
    }

    #[test]
    fn four_player_bipai_accepts_tiles_matching_tile_set() {
        let (tiles, tile_set) = red_three_tiles();

        assert!(Bipai::<FourPlayer>::try_new(tiles, tile_set).is_ok());
    }

    #[test]
    fn newly_constructed_four_player_bipai_has_122_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();

        assert_eq!(bipai.remaining_count(), 122);
    }

    #[test]
    fn qipai_deals_thirteen_expected_tiles_to_each_seat() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, actual): (_, [Bingpai; 4]) = bipai.qipai();

        let mut expected = [[0; 37]; 4];
        expected[0][TileKind::M1.index()] = 4;
        expected[0][TileKind::M5.index()] = 3;
        expected[0][TileKind::M6.index()] = 1;
        expected[0][TileKind::M9.index()] = 3;
        expected[0][TileKind::P1.index()] = 1;
        expected[0][TileKind::P4.index()] = 1;

        expected[1][TileKind::M2.index()] = 4;
        expected[1][TileKind::M6.index()] = 3;
        expected[1][TileKind::M7.index()] = 1;
        expected[1][TileKind::P1.index()] = 3;
        expected[1][TileKind::P2.index()] = 1;
        expected[1][TileKind::P4.index()] = 1;

        expected[2][TileKind::M3.index()] = 4;
        expected[2][TileKind::M7.index()] = 3;
        expected[2][TileKind::M8.index()] = 1;
        expected[2][TileKind::P2.index()] = 3;
        expected[2][TileKind::P3.index()] = 1;
        expected[2][TileKind::P4.index()] = 1;

        expected[3][TileKind::M4.index()] = 4;
        expected[3][TileKind::M8.index()] = 3;
        expected[3][TileKind::M9.index()] = 1;
        expected[3][TileKind::P3.index()] = 3;
        expected[3][TileKind::P4.index()] = 1;
        expected[3][TileKind::P5.index()] = 1;

        assert_eq!(actual.map(|bingpai| *bingpai.counts()), expected);
    }

    #[test]
    fn qipai_leaves_70_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (bipai, _) = bipai.qipai();

        assert_eq!(bipai.remaining_count(), 70);
    }

    #[test]
    fn qipai_bingpai_preserves_tile_set_limit() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (_, bingpai) = bipai.qipai();

        assert_eq!(
            bingpai[0].clone().with_added(TileKind::M5),
            Err(crate::bingpai::BingpaiError::TileCountExceeded {
                tile_kind: TileKind::M5,
                max_count: 3,
            }),
        );
    }

    #[test]
    fn first_zimo_after_qipai_returns_tile_at_index_52() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (bipai, _) = bipai.qipai();
        let (_, zimopai) = bipai.zimo().unwrap();

        assert_eq!(zimopai, TileKind::P5);
    }

    #[test]
    fn first_zimo_after_qipai_leaves_69_remaining_tiles() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (bipai, _) = bipai.qipai();
        let (bipai, _) = bipai.zimo().unwrap();

        assert_eq!(bipai.remaining_count(), 69);
    }

    #[test]
    fn consecutive_zimo_after_qipai_preserves_wall_order() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (bipai, _) = bipai.qipai();
        let (bipai, first) = bipai.zimo().unwrap();
        let (bipai, second) = bipai.zimo().unwrap();
        let (bipai, third) = bipai.zimo().unwrap();
        let (_, fourth) = bipai.zimo().unwrap();

        assert_eq!(
            [first, second, third, fourth],
            [TileKind::P5, TileKind::P5, TileKind::P6, TileKind::P6],
        );
    }

    #[test]
    fn zimo_rejects_draw_after_live_wall_is_exhausted() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let (mut bipai, _) = bipai.qipai();
        for _ in 0..70 {
            (bipai, _) = bipai.zimo().unwrap();
        }

        assert_eq!(bipai.zimo(), Err(BipaiError::LiveWallExhausted));
    }

    #[test]
    fn four_player_bipai_rejects_tile_set_with_one_fewer_tile() {
        let (tiles, _) = red_three_tiles();
        let mut counts = [4; 37];
        counts[TileKind::M1.index()] = 3;
        counts[TileKind::M0.index()] = 1;
        counts[TileKind::M5.index()] = 3;
        counts[TileKind::P0.index()] = 1;
        counts[TileKind::P5.index()] = 3;
        counts[TileKind::S0.index()] = 1;
        counts[TileKind::S5.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(
            Bipai::<FourPlayer>::try_new(tiles, tile_set),
            Err(BipaiError::TileSetMismatch {
                tile_kind: TileKind::M1,
                actual_count: 4,
                expected_count: 3,
            }),
        );
    }

    #[test]
    fn four_player_bipai_rejects_tile_set_with_one_more_tile() {
        let (tiles, _) = red_three_tiles();
        let mut counts = [4; 37];
        counts[TileKind::M0.index()] = 0;
        counts[TileKind::M5.index()] = 4;
        counts[TileKind::P0.index()] = 1;
        counts[TileKind::P5.index()] = 3;
        counts[TileKind::S0.index()] = 1;
        counts[TileKind::S5.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(
            Bipai::<FourPlayer>::try_new(tiles, tile_set),
            Err(BipaiError::TileSetMismatch {
                tile_kind: TileKind::M5,
                actual_count: 3,
                expected_count: 4,
            }),
        );
    }

    #[test]
    fn four_player_bipai_rejects_duplicate_tile_kind_when_counts_differ() {
        let (mut tiles, tile_set) = red_three_tiles();
        tiles[0] = TileKind::M2;

        assert_eq!(
            Bipai::<FourPlayer>::try_new(tiles, tile_set),
            Err(BipaiError::TileSetMismatch {
                tile_kind: TileKind::M1,
                actual_count: 3,
                expected_count: 4,
            }),
        );
    }

    #[test]
    fn four_player_bipai_rejects_excess_tile_kind_when_counts_differ() {
        let (mut tiles, tile_set) = red_three_tiles();
        tiles[4] = TileKind::M1;

        assert_eq!(
            Bipai::<FourPlayer>::try_new(tiles, tile_set),
            Err(BipaiError::TileSetMismatch {
                tile_kind: TileKind::M1,
                actual_count: 5,
                expected_count: 4,
            }),
        );
    }

    #[test]
    fn four_player_bipai_accepts_shuffled_red_and_base_five_order() {
        let (mut tiles, tile_set) = red_three_tiles();
        let red_index = tiles
            .iter()
            .position(|tile_kind| *tile_kind == TileKind::M0)
            .unwrap();
        let base_index = tiles
            .iter()
            .position(|tile_kind| *tile_kind == TileKind::M5)
            .unwrap();
        tiles.swap(red_index, base_index);

        assert!(Bipai::<FourPlayer>::try_new(tiles, tile_set).is_ok());
    }
}
