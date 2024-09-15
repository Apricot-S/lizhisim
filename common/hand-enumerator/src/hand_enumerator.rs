use std::{cell::Cell, rc::Rc};

pub struct HandEnumerator {
    tiles: Vec<usize>,
    current_hand: Vec<u8>,
    stack: Vec<(usize, Rc<Cell<usize>>)>,
    length: usize,
}

impl HandEnumerator {
    pub fn new(length: usize) -> Result<Self, String> {
        if length < 1 || length > 14 {
            return Err(format!("Hand length must be between 1 and 14.: {}", length));
        }

        Ok(Self {
            tiles: vec![4; 34],
            current_hand: Vec::new(),
            stack: vec![(0, Rc::new(Cell::new(0)))],
            length,
        })
    }
}

impl Iterator for HandEnumerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, i)) = self.stack.last().cloned() {
            if self.current_hand.len() == self.length {
                let result = self.current_hand.clone();
                // バックトラック処理
                self.stack.pop();
                if let Some(last_tile) = self.current_hand.pop() {
                    self.tiles[last_tile as usize] += 1;
                }
                return Some(result);
            }

            if i.get() >= 34 {
                // このレベルでのループ終了、バックトラック
                self.stack.pop();
                if let Some(last_tile) = self.current_hand.pop() {
                    self.tiles[last_tile as usize] += 1;
                }
                continue;
            }

            if self.tiles[i.get()] > 0 {
                // タイルを選択
                self.tiles[i.get()] -= 1;
                self.current_hand.push(i.get() as u8);

                // 新しいフレームをスタックにプッシュ（再帰呼び出しのシミュレーション）
                self.stack.push((i.get(), Rc::new(Cell::new(i.get()))));
                // 現在のフレームのインデックスを更新
                i.set(i.get() + 1);
            } else {
                // 次のタイルへ
                i.set(i.get() + 1);
            }
        }

        None
    }
}
