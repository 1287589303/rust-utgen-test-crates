// Answer 0

#[test]
fn test_set_is_from_word_with_single_byte_zero() {
    let mut state_builder = StateBuilderMatches(vec![0]);
    state_builder.set_is_from_word();
}

#[test]
fn test_set_is_from_word_with_single_byte_one() {
    let mut state_builder = StateBuilderMatches(vec![1]);
    state_builder.set_is_from_word();
}

#[test]
fn test_set_is_from_word_with_single_byte_two() {
    let mut state_builder = StateBuilderMatches(vec![2]);
    state_builder.set_is_from_word();
}

#[test]
fn test_set_is_from_word_with_multiple_bytes() {
    let mut state_builder = StateBuilderMatches(vec![0, 1, 2, 3]);
    state_builder.set_is_from_word();
}

#[test]
fn test_set_is_from_word_with_max_possible_byte() {
    let mut state_builder = StateBuilderMatches(vec![255]);
    state_builder.set_is_from_word();
}

#[test]
fn test_set_is_from_word_with_two_bytes_set() {
    let mut state_builder = StateBuilderMatches(vec![128, 255]);
    state_builder.set_is_from_word();
}

