pub fn generate_all_pure_hand(length: usize) -> Result<Vec<Vec<u8>>, String> {
    if length < 1 || length > 14 {
        return Err(format!("Hand length must be between 1 and 14.: {}", length));
    }

    let mut result = Vec::new();
    let mut tiles = [4; 34];
    let mut current_hand = Vec::with_capacity(length);
    backtrack(&mut result, &mut current_hand, &mut tiles, length, 0);
    Ok(result)
}

fn backtrack(
    result: &mut Vec<Vec<u8>>,
    current_hand: &mut Vec<u8>,
    tiles: &mut [u8; 34],
    length: usize,
    start: usize,
) {
    if current_hand.len() == length {
        result.push(current_hand.clone());
        return;
    }

    for i in start..34 {
        if tiles[i] > 0 {
            tiles[i] -= 1;
            current_hand.push(i as u8);
            backtrack(result, current_hand, tiles, length, i);
            current_hand.pop();
            tiles[i] += 1;
        }
    }
}
