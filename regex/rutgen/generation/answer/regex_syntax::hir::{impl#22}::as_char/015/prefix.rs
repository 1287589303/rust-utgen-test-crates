// Answer 0

#[test]
fn test_look_as_char_endlf() {
    let look = Look::EndLF;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_start() {
    let look = Look::Start;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_end() {
    let look = Look::End;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_startlf() {
    let look = Look::StartLF;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_startcrlf() {
    let look = Look::StartCRLF;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_endcrlf() {
    let look = Look::EndCRLF;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordascii() {
    let look = Look::WordAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordasciinegate() {
    let look = Look::WordAsciiNegate;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordunicode() {
    let look = Look::WordUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordunicodenegate() {
    let look = Look::WordUnicodeNegate;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordstartascii() {
    let look = Look::WordStartAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordendascii() {
    let look = Look::WordEndAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordstartunicode() {
    let look = Look::WordStartUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordendunicode() {
    let look = Look::WordEndUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordstarthalfascii() {
    let look = Look::WordStartHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordendhalfascii() {
    let look = Look::WordEndHalfAscii;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordstarthalfunicode() {
    let look = Look::WordStartHalfUnicode;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_wordendhalfunicode() {
    let look = Look::WordEndHalfUnicode;
    let _ = look.as_char();
}

