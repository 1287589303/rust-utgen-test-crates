// Answer 0

#[test]
fn test_is_from_word_true() {
    let input = Repr(&[0b00000100]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_zero() {
    let input = Repr(&[0b00000000]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_one() {
    let input = Repr(&[0b00000001]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_two() {
    let input = Repr(&[0b00000010]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_three() {
    let input = Repr(&[0b00000011]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_four() {
    let input = Repr(&[0b00000101]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_five() {
    let input = Repr(&[0b00000110]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_six() {
    let input = Repr(&[0b00000111]);
    let result = input.is_from_word();
}

#[test]
fn test_is_from_word_false_max() {
    let input = Repr(&[0b11111111]);
    let result = input.is_from_word();
}

