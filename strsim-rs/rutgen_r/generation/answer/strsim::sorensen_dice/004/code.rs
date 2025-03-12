// Answer 0

#[test]
fn test_sorensen_dice_equal_bigrams() {
    assert_eq!(1.0, sorensen_dice("ab", "ab"));
}

#[test]
fn test_sorensen_dice_different_bigrams() {
    assert_eq!(0.0, sorensen_dice("ab", "cd"));
}

#[test]
fn test_sorensen_dice_partial_overlap() {
    assert_eq!(0.5, sorensen_dice("ab", "ac"));
}

#[test]
fn test_sorensen_dice_one_bigram_in_common() {
    assert_eq!(1.0 / 3.0, sorensen_dice("ab", "bc"));
}

#[test]
fn test_sorensen_dice_no_bigrams_in_common() {
    assert_eq!(0.0, sorensen_dice("ab", ""));
}

#[test]
fn test_sorensen_dice_single_char_different() {
    assert_eq!(0.0, sorensen_dice("a", "b"));
}

