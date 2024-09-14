use rand::seq::SliceRandom;
use rand::Rng;

pub fn generate_random_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let mut wall = [0u8; 136];
    wall.iter_mut()
        .enumerate()
        .for_each(|(i, tile)| *tile = (i / 4) as u8);
    wall.shuffle(rng);

    const CHOICES: [usize; 10] = [1, 2, 4, 5, 7, 8, 10, 11, 13, 14];
    let hand_length = *CHOICES.choose(rng).unwrap();

    let mut hand = [0u8; 34];
    wall.iter()
        .take(hand_length)
        .for_each(|&t| hand[t as usize] += 1);

    hand
}
