// Answer 0

#[test]
fn test_sorensen_dice_equal_empty_strings() {
    assert_eq!(1.0, sorensen_dice("", ""));
}

#[test]
fn test_sorensen_dice_equal_same_strings() {
    assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
}

#[test]
fn test_sorensen_dice_equal_with_whitespace() {
    assert_eq!(1.0, sorensen_dice(" ferris ", " ferris "));
}

#[test]
fn test_sorensen_dice_equal_with_different_case() {
    assert_eq!(1.0, sorensen_dice("Ferris", "ferris"));
}

