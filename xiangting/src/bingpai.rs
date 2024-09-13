use crate::constants::{MAX_NUM_SAME_TILE, MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use thiserror::Error;

pub type Bingpai = [u8; NUM_TILE_INDEX];

#[derive(Debug, Error)]
pub enum InvalidBingpaiError {
    #[error("Invalid hand: Same tile count exceeds 4 ({0}).")]
    ExceedsMaxNumSameTile(u8),
    #[error("Invalid hand: Total tile count exceeds 14 ({0}).")]
    ExceedsMaxNumBingpai(u8),
    #[error("Invalid hand: Hand is empty.")]
    EmptyBingpai,
}

pub(super) fn count_bingpai(bingpai: &Bingpai) -> Result<u8, InvalidBingpaiError> {
    let num_bingpai = bingpai.iter().try_fold(0, |acc, &num_tile| {
        if num_tile > MAX_NUM_SAME_TILE {
            return Err(InvalidBingpaiError::ExceedsMaxNumSameTile(num_tile));
        }
        Ok(acc + num_tile)
    })?;

    if num_bingpai > MAX_NUM_SHOUPAI {
        return Err(InvalidBingpaiError::ExceedsMaxNumBingpai(num_bingpai));
    }
    if num_bingpai == 0 {
        return Err(InvalidBingpaiError::EmptyBingpai);
    }

    Ok(num_bingpai)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_bingpai() {
        let bingpai_1: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let bingpai_2: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 1, 1, 1, 1, 1, // z
        ];
        let bingpai_3: Bingpai = [
            4, 4, 4, 2, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];

        let num_bingpai_1 = count_bingpai(&bingpai_1).unwrap();
        let num_bingpai_2 = count_bingpai(&bingpai_2).unwrap();
        let num_bingpai_3 = count_bingpai(&bingpai_3).unwrap();

        assert_eq!(num_bingpai_1, bingpai_1.iter().sum());
        assert_eq!(num_bingpai_2, bingpai_2.iter().sum());
        assert_eq!(num_bingpai_3, bingpai_3.iter().sum());
    }

    #[test]
    fn invalid_bingpai() {
        let bingpai_1: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let bingpai_2: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 1, 1, 1, 1, 1, 1, // z
        ];
        let bingpai_3: Bingpai = [
            5, 4, 4, 2, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let bingpai_4: Bingpai = [
            5, 4, 4, 4, 2, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];

        let num_bingpai_1 = count_bingpai(&bingpai_1).unwrap_err();
        let num_bingpai_2 = count_bingpai(&bingpai_2).unwrap_err();
        let num_bingpai_3 = count_bingpai(&bingpai_3).unwrap_err();
        let num_bingpai_4 = count_bingpai(&bingpai_4).unwrap_err();

        assert!(matches!(num_bingpai_1, InvalidBingpaiError::EmptyBingpai));
        assert!(matches!(
            num_bingpai_2,
            InvalidBingpaiError::ExceedsMaxNumBingpai(15)
        ));
        assert!(matches!(
            num_bingpai_3,
            InvalidBingpaiError::ExceedsMaxNumSameTile(5)
        ));
        assert!(matches!(
            num_bingpai_4,
            InvalidBingpaiError::ExceedsMaxNumSameTile(5)
        ));
    }
}
