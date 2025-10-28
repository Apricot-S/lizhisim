// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::bingpai::BingpaiError;
use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianzi, FuluMianziError};
use crate::tile::TileCounts;
use thiserror::Error;

type FuluMianziList = [FuluMianzi];

trait FuluMianziListExt {
    fn to_tile_counts(&self) -> TileCounts;
}

impl FuluMianziListExt for FuluMianziList {
    fn to_tile_counts(&self) -> TileCounts {
        self.iter().fold([0u8; 34], |mut fulupai, m| {
            match m {
                FuluMianzi::Shunzi(t, ClaimedTilePosition::Low) => {
                    fulupai[*t as usize] += 1;
                    fulupai[(t + 1) as usize] += 1;
                    fulupai[(t + 2) as usize] += 1;
                }
                FuluMianzi::Shunzi(t, ClaimedTilePosition::Middle) => {
                    fulupai[(t - 1) as usize] += 1;
                    fulupai[*t as usize] += 1;
                    fulupai[(t + 1) as usize] += 1;
                }
                FuluMianzi::Shunzi(t, ClaimedTilePosition::High) => {
                    fulupai[(t - 2) as usize] += 1;
                    fulupai[(t - 1) as usize] += 1;
                    fulupai[*t as usize] += 1;
                }
                FuluMianzi::Kezi(t) => {
                    fulupai[*t as usize] += 3;
                }
                FuluMianzi::Angangzi(t) | FuluMianzi::Minggangzi(t) => {
                    fulupai[*t as usize] += 4;
                }
            }
            fulupai
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;

    #[test]
    fn to_tile_counts_1m_23m() {
        let fulu_mianzi_list = [FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("123m")
        );
    }

    #[test]
    fn to_tile_counts_4p_35p() {
        let fulu_mianzi_list = [FuluMianzi::Shunzi(12, ClaimedTilePosition::Middle)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("435p")
        );
    }

    #[test]
    fn to_tile_counts_9s_78s() {
        let fulu_mianzi_list = [FuluMianzi::Shunzi(26, ClaimedTilePosition::High)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("978s")
        );
    }

    #[test]
    fn to_tile_counts_111z() {
        let fulu_mianzi_list = [FuluMianzi::Kezi(27)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("111z")
        );
    }

    #[test]
    fn to_tile_counts_concealed_7777z() {
        let fulu_mianzi_list = [FuluMianzi::Angangzi(33)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("7777z")
        );
    }

    #[test]
    fn to_tile_counts_open_7777z() {
        let fulu_mianzi_list = [FuluMianzi::Minggangzi(33)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("7777z")
        );
    }

    #[test]
    #[should_panic]
    fn to_tile_counts_111z_not_consider_invalid_fulu() {
        let fulu_mianzi_list = [FuluMianzi::Kezi(34)];
        fulu_mianzi_list.to_tile_counts();
    }
}
