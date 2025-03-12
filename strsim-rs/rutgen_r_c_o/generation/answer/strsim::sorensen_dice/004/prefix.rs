// Answer 0

#[test]
fn test_sorensen_dice_different_bigrams() {
    let result = sorensen_dice("ab", "ac");
}

#[test]
fn test_sorensen_dice_permuted_bigrams() {
    let result = sorensen_dice("ab", "ba");
}

#[test]
fn test_sorensen_dice_no_common_bigrams() {
    let result = sorensen_dice("ab", "xx");
}

#[test]
fn test_sorensen_dice_completely_different_bigrams() {
    let result = sorensen_dice("ab", "cd");
}

#[test]
fn test_sorensen_dice_reverse_bigrams() {
    let result = sorensen_dice("cd", "ab");
}

#[test]
fn test_sorensen_dice_one_common_char() {
    let result = sorensen_dice("ab", "aa");
}

#[test]
fn test_sorensen_dice_other_order_common_char() {
    let result = sorensen_dice("aa", "ab");
}

#[test]
fn test_sorensen_dice_space_and_char() {
    let result = sorensen_dice("ab", "a ");
}

#[test]
fn test_sorensen_dice_leading_space() {
    let result = sorensen_dice(" a", "a ");
}

#[test]
fn test_sorensen_dice_ab_vs_empty_spaces() {
    let result = sorensen_dice("ab", " ");
}

