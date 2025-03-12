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
fn test_sorensen_dice_single_bigram() {
    assert_eq!(0.5, sorensen_dice("ab", "ac"));
}

#[test]
fn test_sorensen_dice_no_bigrams() {
    assert_eq!(0.0, sorensen_dice("ab", "e"));
}

#[test]
fn test_sorensen_dice_one_bigram_match() {
    assert_eq!(0.6666666666666666, sorensen_dice("ab", "bc"));
}

#[test]
fn test_sorensen_dice_no_common_bigrams() {
    assert_eq!(0.0, sorensen_dice("cd", "ef"));
}

