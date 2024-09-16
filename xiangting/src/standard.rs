use super::bingpai::Bingpai;
use super::constants::{MAX_NUM_FULU_MIANZI, NUM_TILE_INDEX};
use super::mianzi::Mianzi;
use super::shoupai::count_fulupai;
use bitvec::prelude::*;

struct MianziDaziGulipai {
    num_mianzi: u8,
    num_dazi: u8,
    num_gulipai: u8,
}

struct MianziDaziGulipaiPattern {
    a: MianziDaziGulipai,
    b: MianziDaziGulipai,
}

type TileFlag = BitArr!(for NUM_TILE_INDEX);

fn count_4_tiles_in_shoupai(
    bingpai: &Bingpai,
    fulu_mianzi: &Option<[Option<Mianzi>; MAX_NUM_FULU_MIANZI]>,
) -> TileFlag {
    let mut result: TileFlag = BitArray::ZERO;
    match fulu_mianzi {
        None => {
            bingpai
                .iter()
                .enumerate()
                .for_each(|(i, &num_tile_bingpai)| {
                    if num_tile_bingpai == 4 {
                        result.set(i, true);
                    }
                });
        }
        Some(f) => {
            let fulupai = count_fulupai(f);
            bingpai.iter().zip(fulupai.iter()).enumerate().for_each(
                |(i, (&num_tile_bingpai, &num_tile_fulupai))| {
                    if (num_tile_bingpai + num_tile_fulupai) == 4 {
                        result.set(i, true);
                    }
                },
            );
        }
    }
    result
}

fn calculate_replacement_number_formula(
    mut num_mianzi: u8,
    mut num_dazi: u8,
    mut num_gulipai: u8,
    has_jiangpai: bool,
) -> u8 {
    // If there is no pair, 5 blocks are needed
    let num_required_block: u8 = if has_jiangpai { 4 } else { 5 };

    // Adjust for excess melds
    if num_mianzi > 4 {
        num_dazi += num_mianzi - 4;
        num_mianzi = 4;
    }

    // Adjust for excess meld candidates
    if (num_mianzi + num_dazi) > 4 {
        num_gulipai += num_mianzi + num_dazi - 4;
        num_dazi = 4 - num_mianzi;
    }

    // Adjust for excess isolated tiles
    if (num_mianzi + num_dazi + num_gulipai) > num_required_block {
        num_gulipai = num_required_block - num_mianzi - num_dazi;
    }

    // Count the pair as a meld candidate if it exists
    if has_jiangpai {
        num_dazi += 1;
    }

    14 - num_mianzi * 3 - num_dazi * 2 - num_gulipai
}

fn count_shupai_dazi_gulipai(bingpai: &[u8]) -> MianziDaziGulipaiPattern {
    let mut num_tile: u8 = 0;
    let mut num_dazi: u8 = 0;
    let mut num_gulipai: u8 = 0;

    for n in 0..9 {
        num_tile += bingpai[n];

        if (n <= 6) && (bingpai[n + 1] == 0) && (bingpai[n + 2] == 0) {
            num_dazi += num_tile >> 1;
            num_gulipai += num_tile % 2;
            num_tile = 0;
        }
    }

    num_dazi += num_tile >> 1;
    num_gulipai += num_tile % 2;

    MianziDaziGulipaiPattern {
        a: MianziDaziGulipai {
            num_mianzi: 0,
            num_dazi,
            num_gulipai,
        },
        b: MianziDaziGulipai {
            num_mianzi: 0,
            num_dazi,
            num_gulipai,
        },
    }
}

fn count_shupai_mianzi_dazi_gulipai(bingpai: &mut [u8], n: usize) -> MianziDaziGulipaiPattern {
    if n > 8 {
        return count_shupai_dazi_gulipai(bingpai);
    }

    let mut max = count_shupai_mianzi_dazi_gulipai(bingpai, n + 1);

    #[inline]
    fn update_max(max: &mut MianziDaziGulipaiPattern, r: MianziDaziGulipaiPattern) {
        if (r.a.num_gulipai < max.a.num_gulipai)
            || (r.a.num_gulipai == max.a.num_gulipai) && (r.a.num_dazi < max.a.num_dazi)
        {
            max.a = r.a;
        }
        if (r.b.num_mianzi > max.b.num_mianzi)
            || (r.b.num_mianzi == max.b.num_mianzi) && (r.b.num_dazi > max.b.num_dazi)
        {
            max.b = r.b;
        }
    }

    if (n <= 6) && (bingpai[n] > 0) && (bingpai[n + 1] > 0) && (bingpai[n + 2] > 0) {
        bingpai[n] -= 1;
        bingpai[n + 1] -= 1;
        bingpai[n + 2] -= 1;
        let mut r = count_shupai_mianzi_dazi_gulipai(bingpai, n);
        bingpai[n] += 1;
        bingpai[n + 1] += 1;
        bingpai[n + 2] += 1;

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if bingpai[n] >= 3 {
        bingpai[n] -= 3;
        let mut r = count_shupai_mianzi_dazi_gulipai(bingpai, n);
        bingpai[n] += 3;

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    max
}

fn count_zipai_mianzi_dazi_gulipai(bingpai: &[u8]) -> MianziDaziGulipai {
    bingpai.iter().fold(
        MianziDaziGulipai {
            num_mianzi: 0,
            num_dazi: 0,
            num_gulipai: 0,
        },
        |mut acc, &n| {
            if n >= 3 {
                acc.num_mianzi += 1;
            } else if n == 2 {
                acc.num_dazi += 1;
            } else if n == 1 {
                acc.num_gulipai += 1;
            }
            acc
        },
    )
}

fn calculate_replacement_number_inner(
    bingpai: &mut Bingpai,
    num_bingpai: u8,
    has_jiangpai: bool,
) -> u8 {
    let r_wanzi = count_shupai_mianzi_dazi_gulipai(&mut bingpai[0..9], 0);
    let r_bingzi = count_shupai_mianzi_dazi_gulipai(&mut bingpai[9..18], 0);
    let r_suozi = count_shupai_mianzi_dazi_gulipai(&mut bingpai[18..27], 0);
    let z = count_zipai_mianzi_dazi_gulipai(&bingpai[27..34]);

    let num_fulu = match num_bingpai {
        12..=14 => 0,
        9..=11 => 1,
        6..=8 => 2,
        3..=5 => 3,
        1..=2 => 4,
        _ => panic!("Invalid hand."),
    };

    let mut min = 14;

    for m in [&r_wanzi.a, &r_wanzi.b] {
        for p in [&r_bingzi.a, &r_bingzi.b] {
            for s in [&r_suozi.a, &r_suozi.b] {
                let num_mianzi =
                    num_fulu + m.num_mianzi + p.num_mianzi + s.num_mianzi + z.num_mianzi;
                let num_dazi = m.num_dazi + p.num_dazi + s.num_dazi + z.num_dazi;
                let num_gulipai = m.num_gulipai + p.num_gulipai + s.num_gulipai + z.num_gulipai;

                let replacement_number = calculate_replacement_number_formula(
                    num_mianzi,
                    num_dazi,
                    num_gulipai,
                    has_jiangpai,
                );
                if replacement_number < min {
                    min = replacement_number;
                }
            }
        }
    }

    min
}

pub(crate) fn calculate_replacement_number(
    mut bingpai: Bingpai,
    fulu_mianzi: &Option<[Option<Mianzi>; MAX_NUM_FULU_MIANZI]>,
    num_bingpai: u8,
) -> u8 {
    match fulu_mianzi {
        None => (),
        _ => unimplemented!("Shanten number calculation including melds is not yet supported."),
    }

    // Calculate the replacement number without a pair
    let mut min = calculate_replacement_number_inner(&mut bingpai, num_bingpai, false);

    // Remove a possible pair and calculate the replacement number with a pair
    for n in 0..NUM_TILE_INDEX {
        if bingpai[n] >= 2 {
            bingpai[n] -= 2;
            let replacement_number =
                calculate_replacement_number_inner(&mut bingpai, num_bingpai, true);
            bingpai[n] += 2;
            if replacement_number < min {
                min = replacement_number;
            }
        }
    }

    min
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_replacement_number_formula_works() {
        assert_eq!(calculate_replacement_number_formula(0, 0, 14, false), 9);
        assert_eq!(calculate_replacement_number_formula(4, 0, 0, true), 0);
        assert_eq!(calculate_replacement_number_formula(3, 1, 0, true), 1);
        assert_eq!(calculate_replacement_number_formula(4, 1, 0, false), 1);
        assert_eq!(calculate_replacement_number_formula(4, 0, 2, false), 1);
    }

    #[test]
    fn count_dazi_gulipai_works() {
        let bingpai = [0, 2, 0, 2, 1, 0, 0, 1, 1];
        let r = count_shupai_dazi_gulipai(&bingpai);
        assert_eq!(r.a.num_mianzi, 0);
        assert_eq!(r.a.num_dazi, 3);
        assert_eq!(r.a.num_gulipai, 1);
        assert_eq!(r.b.num_mianzi, 0);
        assert_eq!(r.b.num_dazi, 3);
        assert_eq!(r.b.num_gulipai, 1);
    }

    #[test]
    fn count_mianzi_dazi_gulipai_works() {
        let mut bingpai = [1, 0, 3, 1, 2, 1, 0, 1, 0];
        let r = count_shupai_mianzi_dazi_gulipai(&mut bingpai, 0);
        assert_eq!(r.a.num_mianzi, 1);
        assert_eq!(r.a.num_dazi, 3);
        assert_eq!(r.a.num_gulipai, 0);
        assert_eq!(r.b.num_mianzi, 2);
        assert_eq!(r.b.num_dazi, 0);
        assert_eq!(r.b.num_gulipai, 3);
    }

    #[test]
    #[should_panic]
    fn calculate_replacement_number_empty() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        calculate_replacement_number(bingpai, &None, num_bingpai);
    }

    #[test]
    #[should_panic]
    fn calculate_replacement_number_overdraw() {
        let bingpai: Bingpai = [
            4, 4, 4, 4, 3, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        calculate_replacement_number(bingpai, &None, num_bingpai);
    }

    #[test]
    fn calculate_replacement_number_shisanyao() {
        let bingpai_14: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_1: u8 = bingpai_14.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai_14, &None, num_bingpai_1);
        assert_eq!(replacement_number_1, 8);

        let bingpai_13: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_2: u8 = bingpai_13.iter().sum();
        let replacement_number_2 = calculate_replacement_number(bingpai_13, &None, num_bingpai_2);
        assert_eq!(replacement_number_2, 9);

        let bingpai_12: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_3: u8 = bingpai_12.iter().sum();
        let replacement_number_3 = calculate_replacement_number(bingpai_13, &None, num_bingpai_3);
        assert_eq!(replacement_number_3, 9);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 3, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_with_meld() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            0, 1, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 1, 1, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            1, 1, 0, 0, 0, 0, 1, 1, 1, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_too_many_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 1, 1, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            1, 1, 0, 0, 0, 0, 0, 1, 1, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_not_enough_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
        let bingpai: Bingpai = [
            1, 0, 3, 1, 2, 1, 0, 1, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 1, 1, 0, 0, 1, 2, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand_4_melds_without_a_pair() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 1, 1, 1, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 1, 1, 1, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_triplet_sequence() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 3, 1, 1, 1, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_sequence_isolated_sequence() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 1, 1, 3, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 1, 1, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_pair_triplet_sequence() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            2, 3, 1, 1, 1, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 1, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_pair_sequence_sequence_pair() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 2, 1, 1, 2, 1, 1, 2, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_prioritize_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 面子の分け方
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            1, 0, 3, 1, 2, 1, 0, 1, 0, // s
            2, 0, 0, 0, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            2, 2, 2, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 1); // 現状はアルゴリズム上聴牌と判定される あとで修正する
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_2() {
        let bingpai: Bingpai = [
            4, 1, 1, 4, 0, 0, 0, 0, 0, // m
            4, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 1); // 現状はアルゴリズム上聴牌と判定される あとで修正する
    }
}
