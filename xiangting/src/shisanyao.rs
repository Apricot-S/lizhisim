use super::bingpai::Bingpai;
use super::constants::MAX_NUM_SHOUPAI;

pub(crate) fn calculate_replacement_number(bingpai: &Bingpai, num_bingpai: u8) -> u8 {
    if (num_bingpai != (MAX_NUM_SHOUPAI - 1)) && (num_bingpai != MAX_NUM_SHOUPAI) {
        return u8::MAX;
    }

    let mut num_kinds: u8 = 0;
    let mut has_jiangpai: bool = false;

    const YAOJIUPAI_INDICES: [usize; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
    YAOJIUPAI_INDICES.iter().for_each(|&index| {
        let count = bingpai[index];
        if count >= 1 {
            num_kinds += 1;
            has_jiangpai |= count >= 2;
        }
    });

    14 - num_kinds - (if has_jiangpai { 1 } else { 0 })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_replacement_number_no_terminals_and_honors() {
        let bingpai: Bingpai = [
            0, 1, 1, 1, 2, 0, 0, 0, 0, // m
            0, 0, 1, 1, 1, 0, 0, 0, 0, // p
            0, 0, 0, 1, 1, 1, 1, 1, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 14);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 1, 1, // m
            1, 1, 0, 0, 0, 0, 0, 0, 0, // p
            0, 1, 0, 1, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_with_pair() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 1, 0, 0, 0, 0, 0, 0, 0, // p
            0, 1, 0, 0, 0, 0, 0, 0, 2, // s
            1, 1, 1, 1, 1, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_tenpai_13_wait() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let bingpai_12: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_12: u8 = bingpai_12.iter().sum();
        let replacement_number_12 = calculate_replacement_number(&bingpai_12, num_bingpai_12);
        assert_eq!(replacement_number_12, u8::MAX);

        let bingpai_13: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let replacement_number_13 = calculate_replacement_number(&bingpai_13, 12);
        assert_eq!(replacement_number_13, u8::MAX);
    }
}
