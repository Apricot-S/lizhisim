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
