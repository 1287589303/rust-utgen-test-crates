// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    assert_eq!(1.0, sorensen_dice("test", "test"));
}

#[test]
fn test_sorensen_dice_equal_strings_with_whitespace() {
    assert_eq!(1.0, sorensen_dice(" test ", "test"));
}

#[test]
fn test_sorensen_dice_equal_strings_with_special_characters() {
    assert_eq!(1.0, sorensen_dice("te$t", "te$t"));
}

#[test]
fn test_sorensen_dice_equal_empty_strings() {
    assert_eq!(1.0, sorensen_dice("", ""));
}

#[test]
fn test_sorensen_dice_equal_strings_case_sensitive() {
    assert_eq!(1.0, sorensen_dice("Data", "Data"));
}

