// Answer 0

#[test]
fn test_reversed_end_to_start() {
    let input = Look::End;
    let result = input.reversed();
}

#[test]
fn test_reversed_start_to_end() {
    let input = Look::Start;
    let result = input.reversed();
}

#[test]
fn test_reversed_endlf_to_startlf() {
    let input = Look::EndLF;
    let result = input.reversed();
}

#[test]
fn test_reversed_startlf_to_endlf() {
    let input = Look::StartLF;
    let result = input.reversed();
}

#[test]
fn test_reversed_endcrlf_to_startcrlf() {
    let input = Look::EndCRLF;
    let result = input.reversed();
}

#[test]
fn test_reversed_startcrlf_to_endcrlf() {
    let input = Look::StartCRLF;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordascii_identity() {
    let input = Look::WordAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordasciinegate_identity() {
    let input = Look::WordAsciiNegate;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordunicode_identity() {
    let input = Look::WordUnicode;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordunicodenegate_identity() {
    let input = Look::WordUnicodeNegate;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordstartascii_to_wordendascii() {
    let input = Look::WordStartAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordendascii_to_wordstartascii() {
    let input = Look::WordEndAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordstartunicode_to_wordendunicode() {
    let input = Look::WordStartUnicode;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordendunicode_to_wordstartunicode() {
    let input = Look::WordEndUnicode;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordstarthalfascii_to_wordendhalfascii() {
    let input = Look::WordStartHalfAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordendhalfascii_to_wordstarthalfascii() {
    let input = Look::WordEndHalfAscii;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordstarthalfunicode_to_wordendhalfunicode() {
    let input = Look::WordStartHalfUnicode;
    let result = input.reversed();
}

#[test]
fn test_reversed_wordendhalfunicode_to_wordstarthalfunicode() {
    let input = Look::WordEndHalfUnicode;
    let result = input.reversed();
}

