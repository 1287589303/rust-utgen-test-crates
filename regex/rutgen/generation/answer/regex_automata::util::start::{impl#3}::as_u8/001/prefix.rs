// Answer 0

#[test]
fn test_start_as_u8_non_word_byte() {
    let start = Start::NonWordByte;
    let result = start.as_u8();
}

#[test]
fn test_start_as_u8_word_byte() {
    let start = Start::WordByte;
    let result = start.as_u8();
}

#[test]
fn test_start_as_u8_text() {
    let start = Start::Text;
    let result = start.as_u8();
}

#[test]
fn test_start_as_u8_line_lf() {
    let start = Start::LineLF;
    let result = start.as_u8();
}

#[test]
fn test_start_as_u8_line_cr() {
    let start = Start::LineCR;
    let result = start.as_u8();
}

#[test]
fn test_start_as_u8_custom_line_terminator() {
    let start = Start::CustomLineTerminator;
    let result = start.as_u8();
}

