use super::bingpai::{count_bingpai, Bingpai, InvalidBingpaiError};
use super::constants::MAX_NUM_FULU_MIANZI;
use super::mianzi::Mianzi;
use super::qiduizi;
use super::shisanyao;
use super::shoupai::{validate_shoupai, InvalidShoupaiError};
use super::standard;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("InvalidBingpaiError({0})")]
    InvalidBingpai(#[from] InvalidBingpaiError),
    #[error("InvalidShoupaiError({0})")]
    InvalidShoupai(#[from] InvalidShoupaiError),
}

/// Calculates the replacement number for a given hand.
///
/// # Arguments
///
/// * `bingpai` - A reference to the count of each tile to pure (discardable) hand.
/// * `fulu_mianzi` - An optional reference to an array of optional `Mianzi` representing melds.
///
/// # Returns
///
/// A `Result` containing the replacement number as `u8` or a `XiangtingError`.
///
/// # Examples
///
/// ```
/// # use xiangting::calculate_replacement_number;
/// // 123m456p789s1122z
/// let hand_13: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 2, 0, 0, 0, 0, 0, // z
/// ];
/// let replacement_number_13 = calculate_replacement_number(&hand_13, &None);
/// assert_eq!(replacement_number_13.unwrap(), 1u8);
///
/// // 123m11z (3 melds)
/// let hand_4: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     2, 0, 0, 0, 0, 0, 0, // z
/// ];
/// let replacement_number_4 = calculate_replacement_number(&hand_4, &None);
/// assert_eq!(replacement_number_4.unwrap(), 0u8);
/// ```
pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi: &Option<[Option<Mianzi>; MAX_NUM_FULU_MIANZI]>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = count_bingpai(bingpai)?;

    if let Some(f) = fulu_mianzi {
        validate_shoupai(bingpai, f)?;
    }

    let r0 = standard::calculate_replacement_number(*bingpai, fulu_mianzi, num_bingpai);
    let r1 = qiduizi::calculate_replacement_number(bingpai, num_bingpai);
    let r2 = shisanyao::calculate_replacement_number(bingpai, num_bingpai);
    Ok([r0, r1, r2].into_iter().min().unwrap())
}

/// Calculates the xiangting number for a given hand.
///
/// # Arguments
///
/// * `bingpai` - A reference to the count of each tile to pure (discardable) hand.
/// * `fulu_mianzi` - An optional reference to an array of optional `Mianzi` representing melds.
///
/// # Returns
///
/// A `Result` containing the xiangting number as `i8` or a `XiangtingError`.
///
/// # Examples
///
/// ```
/// # use xiangting::calculate_xiangting_number;
/// // 123m456p789s1122z
/// let hand_13: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 2, 0, 0, 0, 0, 0, // z
/// ];
/// let xiangting_number_13 = calculate_xiangting_number(&hand_13, &None);
/// assert_eq!(xiangting_number_13.unwrap(), 0i8);
///
/// // 123m11z (3 melds)
/// let hand_4: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     2, 0, 0, 0, 0, 0, 0, // z
/// ];
/// let xiangting_number_4 = calculate_xiangting_number(&hand_4, &None);
/// assert_eq!(xiangting_number_4.unwrap(), -1i8);
/// ```
pub fn calculate_xiangting_number(
    bingpai: &Bingpai,
    fulu_mianzi: &Option<[Option<Mianzi>; MAX_NUM_FULU_MIANZI]>,
) -> Result<i8, XiangtingError> {
    Ok((calculate_replacement_number(bingpai, fulu_mianzi)? as i8) - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_replacement_number_standard_tenpai() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_shisanyao_tenpai() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_qiduizi_tenpai() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 2, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_xiangting_number_standard_tenpai() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_xiangting_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 0);
    }

    #[test]
    fn calculate_xiangting_number_shisanyao_tenpai() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let replacement_number = calculate_xiangting_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 0);
    }

    #[test]
    fn calculate_xiangting_number_qiduizi_tenpai() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 2, // z
        ];
        let replacement_number = calculate_xiangting_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 0);
    }
}
