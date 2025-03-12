// Answer 0

#[test]
fn test_reversed_word_end_ascii() {
    let input = Look::WordEndAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_word_start_ascii() {
    let input = Look::WordStartAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_word_end_unicode() {
    let input = Look::WordEndUnicode;
    let result = input.reversed();
}

#[test]
fn test_reversed_word_start_unicode() {
    let input = Look::WordStartUnicode;
    let result = input.reversed();
}

