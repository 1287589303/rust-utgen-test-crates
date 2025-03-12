// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    assert_eq!(1.0, sorensen_dice("aa", "aa"));
}

#[test]
fn test_sorensen_dice_empty_string_vs_non_empty() {
    assert_eq!(0.0, sorensen_dice("aa", "b"));
}

#[test]
fn test_sorensen_dice_two_distinct_bigrams() {
    assert_eq!(0.0, sorensen_dice("ab", "cd"));
}

#[test]
fn test_sorensen_dice_overlap_with_no_common_bigrams() {
    assert_eq!(0.0, sorensen_dice("ab", "ef"));
}

#[test]
fn test_sorensen_dice_identical_two_character_strings() {
    assert_eq!(1.0, sorensen_dice("ab", "ab"));
}

