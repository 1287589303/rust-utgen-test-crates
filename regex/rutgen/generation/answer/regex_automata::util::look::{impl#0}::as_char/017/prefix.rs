// Answer 0

#[test]
fn test_as_char_look_end() {
    let look_end = Look::End;
    let result = look_end.as_char();
}

#[test]
fn test_as_char_look_start() {
    let look_start = Look::Start;
    let result = look_start.as_char();
}

#[test]
fn test_as_char_look_startLF() {
    let look_startLF = Look::StartLF;
    let result = look_startLF.as_char();
}

#[test]
fn test_as_char_look_endLF() {
    let look_endLF = Look::EndLF;
    let result = look_endLF.as_char();
}

#[test]
fn test_as_char_look_startCRLF() {
    let look_startCRLF = Look::StartCRLF;
    let result = look_startCRLF.as_char();
}

#[test]
fn test_as_char_look_endCRLF() {
    let look_endCRLF = Look::EndCRLF;
    let result = look_endCRLF.as_char();
}

#[test]
fn test_as_char_look_wordAscii() {
    let look_wordAscii = Look::WordAscii;
    let result = look_wordAscii.as_char();
}

#[test]
fn test_as_char_look_wordAsciiNegate() {
    let look_wordAsciiNegate = Look::WordAsciiNegate;
    let result = look_wordAsciiNegate.as_char();
}

#[test]
fn test_as_char_look_wordUnicode() {
    let look_wordUnicode = Look::WordUnicode;
    let result = look_wordUnicode.as_char();
}

#[test]
fn test_as_char_look_wordUnicodeNegate() {
    let look_wordUnicodeNegate = Look::WordUnicodeNegate;
    let result = look_wordUnicodeNegate.as_char();
}

#[test]
fn test_as_char_look_wordStartAscii() {
    let look_wordStartAscii = Look::WordStartAscii;
    let result = look_wordStartAscii.as_char();
}

#[test]
fn test_as_char_look_wordEndAscii() {
    let look_wordEndAscii = Look::WordEndAscii;
    let result = look_wordEndAscii.as_char();
}

#[test]
fn test_as_char_look_wordStartUnicode() {
    let look_wordStartUnicode = Look::WordStartUnicode;
    let result = look_wordStartUnicode.as_char();
}

#[test]
fn test_as_char_look_wordEndUnicode() {
    let look_wordEndUnicode = Look::WordEndUnicode;
    let result = look_wordEndUnicode.as_char();
}

#[test]
fn test_as_char_look_wordStartHalfAscii() {
    let look_wordStartHalfAscii = Look::WordStartHalfAscii;
    let result = look_wordStartHalfAscii.as_char();
}

#[test]
fn test_as_char_look_wordEndHalfAscii() {
    let look_wordEndHalfAscii = Look::WordEndHalfAscii;
    let result = look_wordEndHalfAscii.as_char();
}

#[test]
fn test_as_char_look_wordStartHalfUnicode() {
    let look_wordStartHalfUnicode = Look::WordStartHalfUnicode;
    let result = look_wordStartHalfUnicode.as_char();
}

#[test]
fn test_as_char_look_wordEndHalfUnicode() {
    let look_wordEndHalfUnicode = Look::WordEndHalfUnicode;
    let result = look_wordEndHalfUnicode.as_char();
}

