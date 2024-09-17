use super::bingpai::Bingpai;
use super::constants::{MAX_NUM_FULU_MIANZI, MAX_NUM_SAME_TILE, MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use super::mianzi::{ClaimedTilePosition, InvalidMianziError, Mianzi};
use thiserror::Error;

pub(super) fn count_fulupai(fulu_mianzi: &[Option<Mianzi>; MAX_NUM_FULU_MIANZI]) -> Bingpai {
    let mut fulupai: Bingpai = [0; NUM_TILE_INDEX];
    fulu_mianzi.iter().for_each(|m| match m {
        Some(Mianzi::Shunzi(tile, position)) => {
            fulupai[*tile as usize] += 1;
            match position {
                ClaimedTilePosition::Low => {
                    fulupai[(*tile + 1) as usize] += 1;
                    fulupai[(*tile + 2) as usize] += 1;
                }
                ClaimedTilePosition::Middle => {
                    fulupai[(*tile - 1) as usize] += 1;
                    fulupai[(*tile + 1) as usize] += 1;
                }
                ClaimedTilePosition::High => {
                    fulupai[(*tile - 2) as usize] += 1;
                    fulupai[(*tile - 1) as usize] += 1;
                }
            }
        }
        Some(Mianzi::Kezi(tile)) => fulupai[*tile as usize] += 3,
        Some(Mianzi::Gangzi(tile)) => fulupai[*tile as usize] += 4,
        None => (),
    });
    fulupai
}

#[derive(Debug, Error)]
pub enum InvalidShoupaiError {
    #[error("Invalid hand: Same tile count exceeds 4 ({0}).")]
    ExceedsMaxNumSameTile(u8),
    #[error("Invalid hand: Total tile count exceeds 14 ({0}).")]
    ExceedsMaxNumShoupai(u8),
    #[error("Invalid hand: Total tile count is not a multiple of 3 plus 1 or 2 ({0}).")]
    InvalidNumShoupai(u8),
    #[error("InvalidMianziError({0})")]
    InvalidMianzi(#[from] InvalidMianziError),
}

pub(super) fn validate_shoupai(
    bingpai: &Bingpai,
    fulu_mianzi: &[Option<Mianzi>; MAX_NUM_FULU_MIANZI],
) -> Result<(), InvalidShoupaiError> {
    fulu_mianzi
        .iter()
        .flatten()
        .try_for_each(|m| m.validate())?;

    let num_gangzi = fulu_mianzi
        .iter()
        .flatten()
        .filter(|m| matches!(*m, Mianzi::Gangzi(_)))
        .count() as u8;

    let mut shoupai = *bingpai;
    let fulupai = count_fulupai(fulu_mianzi);
    shoupai
        .iter_mut()
        .zip(fulupai.iter())
        .for_each(|(s, &f)| *s += f);

    let num_shoupai = shoupai.iter().try_fold(0, |acc, &num_tile| {
        if num_tile > MAX_NUM_SAME_TILE {
            return Err(InvalidShoupaiError::ExceedsMaxNumSameTile(num_tile));
        }
        Ok(acc + num_tile)
    })?;

    if num_shoupai > (MAX_NUM_SHOUPAI + num_gangzi) {
        return Err(InvalidShoupaiError::ExceedsMaxNumShoupai(num_shoupai));
    }
    if (num_shoupai - num_gangzi) % 3 == 0 {
        return Err(InvalidShoupaiError::InvalidNumShoupai(num_shoupai));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mianzi::Mianzi;

    #[test]
    fn count_fulupai_menqian() {
        let fulupai_menqian_1: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let fulupai_menqian_2 = count_fulupai(&menqian);
        assert_eq!(fulupai_menqian_1, fulupai_menqian_2);
    }

    #[test]
    fn count_fulupai_3_fulu() {
        let fulupai_3_chi_1: Bingpai = [
            2, 2, 2, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let shunzi_3 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(9, ClaimedTilePosition::Low)),
            None,
        ];
        let fulupai_3_chi_2 = count_fulupai(&shunzi_3);
        assert_eq!(fulupai_3_chi_1, fulupai_3_chi_2);

        let fulupai_3_peng_1: Bingpai = [
            0, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let kezi_3 = [
            None,
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        let fulupai_3_peng_2 = count_fulupai(&kezi_3);
        assert_eq!(fulupai_3_peng_1, fulupai_3_peng_2);

        let fulupai_3_gang_1: Bingpai = [
            4, 0, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let gangzi_3 = [
            Some(Mianzi::Gangzi(0)),
            None,
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(3)),
        ];
        let fulupai_3_gang_2 = count_fulupai(&gangzi_3);
        assert_eq!(fulupai_3_gang_1, fulupai_3_gang_2);
    }

    #[test]
    fn count_fulupai_4_fulu() {
        let fulupai_4_chi_1: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let shunzi_4 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(3, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(6, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(9, ClaimedTilePosition::Low)),
        ];
        let fulupai_4_chi_2 = count_fulupai(&shunzi_4);
        assert_eq!(fulupai_4_chi_1, fulupai_4_chi_2);

        let fulupai_4_peng_1: Bingpai = [
            3, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let kezi_4 = [
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        let fulupai_4_peng_2 = count_fulupai(&kezi_4);
        assert_eq!(fulupai_4_peng_1, fulupai_4_peng_2);

        let fulupai_4_gang_1: Bingpai = [
            4, 4, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(1)),
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(3)),
        ];
        let fulupai_4_gang_2 = count_fulupai(&gangzi_4);
        assert_eq!(fulupai_4_gang_1, fulupai_4_gang_2);
    }

    #[test]
    fn valid_shoupai_menqian() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        validate_shoupai(&bingpai, &menqian).unwrap();
    }

    #[test]
    fn invalid_shoupai_menqian_too_many_tiles() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let result = validate_shoupai(&bingpai, &menqian).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumShoupai(15)
        ));
    }

    #[test]
    fn invalid_shoupai_menqian_5th_tile() {
        let bingpai: Bingpai = [
            5, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let result = validate_shoupai(&bingpai, &menqian).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumSameTile(5)
        ));
    }

    #[test]
    fn invalid_shoupai_menqian_incomplete_hand() {
        let bingpai: Bingpai = [
            4, 4, 4, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let result = validate_shoupai(&bingpai, &menqian).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidNumShoupai(12)));
    }

    #[test]
    fn valid_shoupai_4_fulu() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];

        let kezi_4 = [
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        validate_shoupai(&bingpai, &kezi_4).unwrap();

        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(1)),
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(3)),
        ];
        validate_shoupai(&bingpai, &gangzi_4).unwrap();
    }

    #[test]
    fn invalid_shoupai_4_fulu_too_many_tiles() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let kezi_4 = [
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        let result = validate_shoupai(&bingpai, &kezi_4).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumShoupai(15),
        ));
    }

    #[test]
    fn invalid_shoupai_4_fulu_5th_tile() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(1)),
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(2)),
        ];
        let result = validate_shoupai(&bingpai, &gangzi_4).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumSameTile(8),
        ));
    }

    #[test]
    fn invalid_shoupai_4_fulu_incomplete_hand() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let shunzi_4 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            None,
        ];
        let result = validate_shoupai(&bingpai, &shunzi_4).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidNumShoupai(12)));
    }

    #[test]
    fn invalid_shoupai_4_fulu_invalid_mianzi() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let shunzi_3 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(27, ClaimedTilePosition::Low)),
            None,
        ];
        let result = validate_shoupai(&bingpai, &shunzi_3).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidMianzi(_)));
    }
}
