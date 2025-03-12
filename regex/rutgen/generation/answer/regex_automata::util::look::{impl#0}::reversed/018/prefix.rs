// Answer 0

#[test]
fn test_reversed_start() {
    let look = Look::Start;
    let result = look.reversed();
}

#[test]
fn test_reversed_end() {
    let look = Look::End;
    let result = look.reversed();
}

#[test]
fn test_reversed_startlf() {
    let look = Look::StartLF;
    let result = look.reversed();
}

#[test]
fn test_reversed_endlf() {
    let look = Look::EndLF;
    let result = look.reversed();
}

#[test]
fn test_reversed_startcrlf() {
    let look = Look::StartCRLF;
    let result = look.reversed();
}

#[test]
fn test_reversed_endcrlf() {
    let look = Look::EndCRLF;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordascii() {
    let look = Look::WordAscii;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordasciinegate() {
    let look = Look::WordAsciiNegate;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordunicode() {
    let look = Look::WordUnicode;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordunicodenegate() {
    let look = Look::WordUnicodeNegate;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordstartascii() {
    let look = Look::WordStartAscii;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordendascii() {
    let look = Look::WordEndAscii;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordstartunicode() {
    let look = Look::WordStartUnicode;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordendunicode() {
    let look = Look::WordEndUnicode;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordstarthalfascii() {
    let look = Look::WordStartHalfAscii;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordendhalfascii() {
    let look = Look::WordEndHalfAscii;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordstarthalfunicode() {
    let look = Look::WordStartHalfUnicode;
    let result = look.reversed();
}

#[test]
fn test_reversed_wordendhalfunicode() {
    let look = Look::WordEndHalfUnicode;
    let result = look.reversed();
}

