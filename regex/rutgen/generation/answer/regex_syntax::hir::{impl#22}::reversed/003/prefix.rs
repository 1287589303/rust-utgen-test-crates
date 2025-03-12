// Answer 0

#[test]
fn test_reversed_start() {
    let input = Look::Start;
    let output = input.reversed();
}

#[test]
fn test_reversed_end() {
    let input = Look::End;
    let output = input.reversed();
}

#[test]
fn test_reversed_startlf() {
    let input = Look::StartLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_endlf() {
    let input = Look::EndLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_startcrlf() {
    let input = Look::StartCRLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_endcrlf() {
    let input = Look::EndCRLF;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordascii() {
    let input = Look::WordAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordasciinegate() {
    let input = Look::WordAsciiNegate;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordunicode() {
    let input = Look::WordUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordunicodendegate() {
    let input = Look::WordUnicodeNegate;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordstartascii() {
    let input = Look::WordStartAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordendascii() {
    let input = Look::WordEndAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordstartunicode() {
    let input = Look::WordStartUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordendunicode() {
    let input = Look::WordEndUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordstarthalfascii() {
    let input = Look::WordStartHalfAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordendhalfascii() {
    let input = Look::WordEndHalfAscii;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordstarthalfunicode() {
    let input = Look::WordStartHalfUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordendhalfunicode() {
    let input = Look::WordEndHalfUnicode;
    let output = input.reversed();
}

#[test]
fn test_reversed_wordendhalfascii_results_in_wordstarthalfascii() {
    let input = Look::WordEndHalfAscii;
    let output = input.reversed();
}

