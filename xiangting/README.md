# xiangting

A library for calculation of deficiency number (a.k.a. xiangting (向聴) number).

This is a port of [majiang-core/lib/xiangting.js](https://github.com/kobalab/majiang-core/blob/master/lib/xiangting.js) to Rust.  
However, it does not support short hand or long hand.

**Note:** Calculations considering melds are not yet supported.

## Usage

```rust
use xiangting::{calculate_replacement_number, calculate_xiangting_number};

fn main() {
    // 123m456p789s1122z
    hand_13: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
        2, 2, 0, 0, 0, 0, 0, // z
    ];

    let replacement_number_13 = calculate_replacement_number(&hand_13, &None);
    assert_eq!(replacement_number_13.unwrap(), 1u8);

    let xiangting_number_13 = calculate_xiangting_number(&hand_13, &None);
    assert_eq!(xiangting_number_13.unwrap(), 0i8);

    // 123m11z (3 melds)
    hand_4: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        2, 0, 0, 0, 0, 0, 0, // z
    ];

    let replacement_number_4 = calculate_replacement_number(&hand_4, &None);
    assert_eq!(replacement_number_4.unwrap(), 0u8);

    let xiangting_number_4 = calculate_xiangting_number(&hand_4, &None);
    assert_eq!(xiangting_number_4.unwrap(), -1i8);
}
```
