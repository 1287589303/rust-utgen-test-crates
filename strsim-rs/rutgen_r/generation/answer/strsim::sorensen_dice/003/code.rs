// Answer 0

#[test]
fn test_sorensen_dice_a_len_2_b_len_1() {
    let a = "ab";
    let b = "c";
    let result = sorensen_dice(a, b);
    assert_eq!(0.0, result);
}

#[test]
fn test_sorensen_dice_a_len_2_b_len_0() {
    let a = "ab";
    let b = "";
    let result = sorensen_dice(a, b);
    assert_eq!(0.0, result);
}

