// Answer 0

#[test]
fn test_bigrams_empty_string() {
    let input = "";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_one_character() {
    let input = "a";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_two_characters() {
    let input = "ab";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_three_characters() {
    let input = "abc";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_four_characters() {
    let input = "abcd";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_repeating_characters() {
    let input = "aaa";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_special_characters() {
    let input = "!@#";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_longer_string() {
    let input = "abcdefg";
    let result: Vec<_> = bigrams(input).collect();
}

#[test]
fn test_bigrams_large_string() {
    let input = "a".repeat(100);
    let result: Vec<_> = bigrams(&input).collect();
}

