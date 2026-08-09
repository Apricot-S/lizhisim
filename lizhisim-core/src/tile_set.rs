// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::TileKind;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TileSetError {
    #[error("tile kind {tile_kind:?} has {actual_count} copies, exceeding maximum {max_count}")]
    TileCountExceeded {
        tile_kind: TileKind,
        actual_count: u8,
        max_count: u8,
    },
    #[error(
        "combined count of {hong_baopai:?} and {base_tile:?} is {actual_count}, exceeding maximum {max_count}"
    )]
    CombinedFiveCountExceeded {
        hong_baopai: TileKind,
        base_tile: TileKind,
        actual_count: u8,
        max_count: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileSet {
    counts: [u8; 37],
    total_count: u16,
}

impl TileSet {
    pub const fn try_from_counts(counts: [u8; 37]) -> Result<Self, TileSetError> {
        let mut index = 0;
        let mut total_count = 0;
        while index < counts.len() {
            if counts[index] > 4 {
                return Err(TileSetError::TileCountExceeded {
                    tile_kind: TileKind::ALL[index],
                    actual_count: counts[index],
                    max_count: 4,
                });
            }
            total_count += counts[index] as u16;
            index += 1;
        }

        let combined_m5_count = counts[TileKind::M0.index()] + counts[TileKind::M5.index()];
        if combined_m5_count > 4 {
            return Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::M0,
                base_tile: TileKind::M5,
                actual_count: combined_m5_count,
                max_count: 4,
            });
        }

        let combined_p5_count = counts[TileKind::P0.index()] + counts[TileKind::P5.index()];
        if combined_p5_count > 4 {
            return Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::P0,
                base_tile: TileKind::P5,
                actual_count: combined_p5_count,
                max_count: 4,
            });
        }

        let combined_s5_count = counts[TileKind::S0.index()] + counts[TileKind::S5.index()];
        if combined_s5_count > 4 {
            return Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::S0,
                base_tile: TileKind::S5,
                actual_count: combined_s5_count,
                max_count: 4,
            });
        }

        Ok(Self {
            counts,
            total_count,
        })
    }

    pub const fn max_count(&self, tile_kind: TileKind) -> u8 {
        self.counts[tile_kind.index()]
    }

    pub const fn total_count(&self) -> u16 {
        self.total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physically_possible_counts_construct_tile_set() {
        assert!(TileSet::try_from_counts([0; 37]).is_ok());
    }

    #[test]
    fn tile_set_reports_max_count_for_tile_kind() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(tile_set.max_count(TileKind::M1), 3);
    }

    #[test]
    fn tile_set_total_count_is_sum_of_kind_counts() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 4;
        counts[TileKind::M2.index()] = 3;
        let tile_set = TileSet::try_from_counts(counts).unwrap();

        assert_eq!(tile_set.total_count(), 7);
    }

    #[test]
    fn tile_set_rejects_kind_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::M1.index()] = 5;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::TileCountExceeded {
                tile_kind: TileKind::M1,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn tile_set_rejects_combined_m0_and_m5_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::M0.index()] = 2;
        counts[TileKind::M5.index()] = 3;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::M0,
                base_tile: TileKind::M5,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn tile_set_rejects_combined_p0_and_p5_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::P0.index()] = 2;
        counts[TileKind::P5.index()] = 3;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::P0,
                base_tile: TileKind::P5,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn tile_set_rejects_combined_s0_and_s5_count_above_four() {
        let mut counts = [0; 37];
        counts[TileKind::S0.index()] = 2;
        counts[TileKind::S5.index()] = 3;

        assert_eq!(
            TileSet::try_from_counts(counts),
            Err(TileSetError::CombinedFiveCountExceeded {
                hong_baopai: TileKind::S0,
                base_tile: TileKind::S5,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }
}
