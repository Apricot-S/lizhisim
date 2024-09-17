# xiangting

A library for calculation of deficiency number (a.k.a. xiangting (向聴) number).

This is a port of [majiang-core/lib/xiangting.js](https://github.com/kobalab/majiang-core/blob/master/lib/xiangting.js) to Rust.  
However, the following differences apply:

- The original algorithm miscalculated the deficiency number in some cases. These errors have been corrected in this port, but the calculation speed has deteriorated as a result.
- It does not support short hand or long hand.

## Usage

```rust
use xiangting::{
    calculate_replacement_number, calculate_xiangting_number, ClaimedTilePosition, Mianzi,
};

fn main() {
    // 123m456p789s11222z
    let hand_14: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
        2, 3, 0, 0, 0, 0, 0, // z
    ];

    let replacement_number = calculate_replacement_number(&hand_14, &None);
    assert_eq!(replacement_number.unwrap(), 0u8);

    let xiangting_number = calculate_xiangting_number(&hand_14, &None);
    assert_eq!(xiangting_number.unwrap(), -1i8);

    // 123m1z (3 melds)
    let hand_4: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        1, 0, 0, 0, 0, 0, 0, // z
    ];

    // 456p 789s 111z
    let melds = Some([
        Some(Mianzi::Shunzi(12, ClaimedTilePosition::Low)),
        Some(Mianzi::Shunzi(24, ClaimedTilePosition::Low)),
        Some(Mianzi::Kezi(27)),
        None,
    ]);

    let replacement_number_wo_melds = calculate_replacement_number(&hand_4, &None);
    assert_eq!(replacement_number_wo_melds.unwrap(), 1u8);

    let replacement_number_w_melds = calculate_replacement_number(&hand_4, &melds);
    assert_eq!(replacement_number_w_melds.unwrap(), 2u8);

    let xiangting_number_wo_melds = calculate_xiangting_number(&hand_4, &None);
    assert_eq!(xiangting_number_wo_melds.unwrap(), 0i8);

    let xiangting_number_w_melds = calculate_xiangting_number(&hand_4, &melds);
    assert_eq!(xiangting_number_w_melds.unwrap(), 1i8);
}
```
