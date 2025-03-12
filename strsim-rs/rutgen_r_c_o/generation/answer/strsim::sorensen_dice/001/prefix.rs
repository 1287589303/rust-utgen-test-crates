// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    let a = "test";
    let b = "test";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_with_whitespace() {
    let a = " test ";
    let b = "test";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_with_case_difference() {
    let a = "Test";
    let b = "test";
    sorensen_dice(a, b);
}

