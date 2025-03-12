// Answer 0

#[test]
fn test_as_usize_non_word_byte() {
    let start = Start::NonWordByte;
    let _result = start.as_usize();
}

#[test]
fn test_as_usize_word_byte() {
    let start = Start::WordByte;
    let _result = start.as_usize();
}

#[test]
fn test_as_usize_text() {
    let start = Start::Text;
    let _result = start.as_usize();
}

#[test]
fn test_as_usize_line_lf() {
    let start = Start::LineLF;
    let _result = start.as_usize();
}

#[test]
fn test_as_usize_line_cr() {
    let start = Start::LineCR;
    let _result = start.as_usize();
}

#[test]
fn test_as_usize_custom_line_terminator() {
    let start = Start::CustomLineTerminator;
    let _result = start.as_usize();
}

