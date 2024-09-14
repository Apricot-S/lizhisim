use rand::seq::SliceRandom;
use rand::Rng;

pub fn generate_random_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let mut wall: Vec<u8> = (0..34).flat_map(|x| [x; 4]).collect();
    wall.shuffle(rng);

    const CHOICES: [usize; 10] = [1, 2, 4, 5, 7, 8, 10, 11, 13, 14];
    let hand_length = *CHOICES.choose(rng).unwrap();

    let mut hand = [0u8; 34];
    wall.iter()
        .take(hand_length)
        .for_each(|&t| hand[t as usize] += 1);

    hand
}
