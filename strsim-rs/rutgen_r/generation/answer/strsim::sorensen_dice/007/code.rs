// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    assert_eq!(1.0, sorensen_dice("ab", "ab"));
}

#[test]
fn test_sorensen_dice_empty_string_and_non_empty() {
    assert_eq!(0.0, sorensen_dice("", "ab"));
}

#[test]
fn test_sorensen_dice_two_disjoint_strings() {
    assert_eq!(0.0, sorensen_dice("ab", "cd"));
}

#[test]
fn test_sorensen_dice_minimum_non_matching_bigrams() {
    assert_eq!(0.0, sorensen_dice("ab", "xy"));
}

#[test]
fn test_sorensen_dice_single_bigram_match() {
    assert_eq!(1.0, sorensen_dice("ab", "ab"));
}

#[test]
fn test_sorensen_dice_partial_bigram_match() {
    assert_eq!(0.6666666666666666, sorensen_dice("aba", "bac"));
}

