const MAX_NUM_SAME_TILE: u8 = 4;
const NUM_TILE_INDEX: usize = 3 * 9 + 4 + 3;
const MAX_NUM_HAND: usize = 14;

pub struct HandEnumerator {
    tiles: [u8; NUM_TILE_INDEX],
    current_hand: Vec<u8>,
    stack: Vec<usize>,
    length: usize,
}

impl HandEnumerator {
    pub fn new(length: usize) -> Result<Self, String> {
        if length < 1 || length > MAX_NUM_HAND {
            return Err(format!("Hand length must be between 1 and 14.: {}", length));
        }

        Ok(Self {
            tiles: [MAX_NUM_SAME_TILE; NUM_TILE_INDEX],
            current_hand: Vec::new(),
            stack: vec![0],
            length,
        })
    }
}

impl Iterator for HandEnumerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(i) = self.stack.pop() {
            if self.current_hand.len() == self.length {
                let result = self.current_hand.clone();
                // バックトラック処理
                if let Some(last_tile) = self.current_hand.pop() {
                    self.tiles[last_tile as usize] += 1;
                }
                return Some(result);
            }

            if i >= NUM_TILE_INDEX {
                // このレベルでのループ終了、バックトラック
                if let Some(last_tile) = self.current_hand.pop() {
                    self.tiles[last_tile as usize] += 1;
                }
                continue;
            }

            if self.tiles[i] > 0 {
                // タイルを選択
                self.tiles[i] -= 1;
                self.current_hand.push(i as u8);

                // 現在のフレームのインデックスを更新
                self.stack.push(i + 1);
                // 新しいフレームをスタックにプッシュ（再帰呼び出しのシミュレーション）
                self.stack.push(i);
            } else {
                // 次のタイルへ
                self.stack.push(i + 1);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;

    #[test]
    fn valid_length() {
        (1..=MAX_NUM_HAND).into_iter().for_each(|i| {
            let result = HandEnumerator::new(i);
            assert!(result.is_ok())
        });
    }

    #[test]
    fn invalid_length() {
        let result0 = HandEnumerator::new(0);
        assert!(result0.is_err());
        let result15 = HandEnumerator::new(MAX_NUM_HAND + 1);
        assert!(result15.is_err());
    }

    fn correct_hands(
        length: usize,
        expected_first: &[u8],
        expected_last: &[u8],
        expected_count: usize,
    ) {
        let mut generator = HandEnumerator::new(length).unwrap();

        let start = Instant::now();

        let first_hand = generator.next().unwrap();
        let (last_hand, count) = generator.fold((first_hand.clone(), 1), |(_, count), hand| {
            (hand, count + 1)
        });

        let end = start.elapsed();
        println!(
            "Time elapsed in generating hands (length = {}) is: {:?}",
            length, end,
        );

        assert_eq!(first_hand, expected_first);
        assert_eq!(last_hand, expected_last);
        assert_eq!(count, expected_count);
    }

    #[test]
    #[ignore]
    fn correct_hands_1() {
        correct_hands(1, &[0], &[33], 34);
    }

    #[test]
    #[ignore]
    fn correct_hands_2() {
        correct_hands(2, &[0, 0], &[33, 33], 595);
    }

    #[test]
    #[ignore]
    fn correct_hands_3() {
        correct_hands(3, &[0, 0, 0], &[33, 33, 33], 7_140);
    }

    #[test]
    #[ignore]
    fn correct_hands_4() {
        correct_hands(4, &[0, 0, 0, 0], &[33, 33, 33, 33], 66_045);
    }
}
