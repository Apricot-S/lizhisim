use std::collections::VecDeque;

pub struct HandEnumerator {
    length: usize,
    tiles: [u8; 34],
    current_hand: Vec<u8>,
    stack: VecDeque<(usize, usize)>,
}

impl HandEnumerator {
    pub fn new(length: usize) -> Result<Self, String> {
        if length < 1 || length > 14 {
            return Err(format!("Hand length must be between 1 and 14.: {}", length));
        }

        Ok(Self {
            length,
            tiles: [4; 34],
            current_hand: Vec::with_capacity(length),
            stack: VecDeque::from([(0usize, 0usize)]),
        })
    }
}

impl Iterator for HandEnumerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        print!("stack: {:?}, ", self.stack);
        while let Some((i, start)) = self.stack.pop_front() {
            if self.current_hand.len() == self.length {
                let hand = self.current_hand.clone();
                self.current_hand.clear();
                self.stack.push_back((i, start + 1));
                return Some(hand);
            }

            let mut found = false;
            for j in start..34 {
                if self.tiles[j] > 0 {
                    self.tiles[j] -= 1;
                    self.current_hand.push(j as u8);
                    self.stack.push_back((i, j));
                    found = true;
                    break;
                }
            }

            if !found {
                if let Some(last) = self.current_hand.pop() {
                    self.tiles[last as usize] += 1;
                }
            }
        }

        None
    }
}
