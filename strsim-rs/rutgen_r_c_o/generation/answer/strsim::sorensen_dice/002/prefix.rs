// Answer 0

#[test]
fn test_sorensen_dice_a_empty_b_non_empty() {
    let a = "";
    let b = "test";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_a_single_char_b_non_empty() {
    let a = "a";
    let b = "hello";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_a_empty_b_single_char() {
    let a = "";
    let b = "b";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_a_single_char_b_single_char() {
    let a = "c";
    let b = "d";
    sorensen_dice(a, b);
}

