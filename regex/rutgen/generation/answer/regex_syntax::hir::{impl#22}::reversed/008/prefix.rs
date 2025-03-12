// Answer 0

#[test]
fn test_reversed_start() {
    let result = Look::Start.reversed();
}

#[test]
fn test_reversed_end() {
    let result = Look::End.reversed();
}

#[test]
fn test_reversed_startlf() {
    let result = Look::StartLF.reversed();
}

#[test]
fn test_reversed_endlf() {
    let result = Look::EndLF.reversed();
}

#[test]
fn test_reversed_startcrlf() {
    let result = Look::StartCRLF.reversed();
}

#[test]
fn test_reversed_endcrlf() {
    let result = Look::EndCRLF.reversed();
}

#[test]
fn test_reversed_wordascii() {
    let result = Look::WordAscii.reversed();
}

#[test]
fn test_reversed_wordasciinegate() {
    let result = Look::WordAsciiNegate.reversed();
}

#[test]
fn test_reversed_wordunicode() {
    let result = Look::WordUnicode.reversed();
}

#[test]
fn test_reversed_wordunicodenegate() {
    let result = Look::WordUnicodeNegate.reversed();
}

#[test]
fn test_reversed_wordstartascii() {
    let result = Look::WordStartAscii.reversed();
}

#[test]
fn test_reversed_wordendascii() {
    let result = Look::WordEndAscii.reversed();
}

#[test]
fn test_reversed_wordstartunicode() {
    let result = Look::WordStartUnicode.reversed();
}

#[test]
fn test_reversed_wordendunicode() {
    let result = Look::WordEndUnicode.reversed();
}

#[test]
fn test_reversed_wordstarthalfascii() {
    let result = Look::WordStartHalfAscii.reversed();
}

#[test]
fn test_reversed_wordendhalfascii() {
    let result = Look::WordEndHalfAscii.reversed();
}

#[test]
fn test_reversed_wordstarthalfunicode() {
    let result = Look::WordStartHalfUnicode.reversed();
}

#[test]
fn test_reversed_wordendhalfunicode() {
    let result = Look::WordEndHalfUnicode.reversed();
}

