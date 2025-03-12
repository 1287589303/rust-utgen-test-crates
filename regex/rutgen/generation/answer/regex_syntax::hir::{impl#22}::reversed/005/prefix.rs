// Answer 0

#[test]
fn test_reversed_word_end_unicode() {
    let input = Look::WordEndUnicode;
    let _output = input.reversed();
}

#[test]
fn test_reversed_word_start_unicode() {
    let input = Look::WordStartUnicode;
    let _output = input.reversed();
}

#[test]
fn test_reversed_word_asci() {
    let input = Look::WordAscii;
    let _output = input.reversed();
}

#[test]
fn test_reversed_word_ascii_negate() {
    let input = Look::WordAsciiNegate;
    let _output = input.reversed();
}

