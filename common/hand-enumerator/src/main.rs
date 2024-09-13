mod generate_hand;
mod hand_enumerator;

use crate::generate_hand::generate_all_pure_hand;
//use crate::hand_enumerator::HandEnumerator;
use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        let message = format!("Usage: {} <length> <output_file>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    let length: usize = match args[1].parse() {
        Ok(len) => len,
        Err(message) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
        }
    };
    let output_file = &args[2];

    let hands = match generate_all_pure_hand(length) {
        Ok(h) => h,
        Err(message) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
        }
    };

    let mut file = File::create(output_file)?;
    for hand in hands {
        let line = hand
            .iter()
            .map(|tile| tile.to_string())
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "{}", line)?;
    }

    /*let generator = HandEnumerator::new(length).unwrap();
    generator
        .into_iter()
        .enumerate()
        .for_each(|(i, hand)| println!("{}: {:?}", i, hand));*/

    Ok(())
}
