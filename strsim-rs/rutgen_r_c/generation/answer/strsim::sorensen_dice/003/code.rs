// Answer 0

#[test]
fn test_sorensen_dice_a_len_2_b_len_1() {
    let a = "ab";
    let b = "c";
    assert_eq!(0.0, sorensen_dice(a, b));
}

#[test]
fn test_sorensen_dice_a_len_2_b_empty() {
    let a = "ab";
    let b = "";
    assert_eq!(0.0, sorensen_dice(a, b));
}

