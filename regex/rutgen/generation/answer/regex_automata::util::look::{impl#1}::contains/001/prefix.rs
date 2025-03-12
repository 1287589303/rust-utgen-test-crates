// Answer 0

#[test]
fn test_contains_start() {
    let look_set = LookSet { bits: Look::Start.as_repr() };
    let result = look_set.contains(Look::Start);
}

#[test]
fn test_contains_end() {
    let look_set = LookSet { bits: Look::End.as_repr() };
    let result = look_set.contains(Look::End);
}

#[test]
fn test_contains_startlf() {
    let look_set = LookSet { bits: Look::StartLF.as_repr() };
    let result = look_set.contains(Look::StartLF);
}

#[test]
fn test_contains_endlf() {
    let look_set = LookSet { bits: Look::EndLF.as_repr() };
    let result = look_set.contains(Look::EndLF);
}

#[test]
fn test_contains_startcrlf() {
    let look_set = LookSet { bits: Look::StartCRLF.as_repr() };
    let result = look_set.contains(Look::StartCRLF);
}

#[test]
fn test_contains_endcrlf() {
    let look_set = LookSet { bits: Look::EndCRLF.as_repr() };
    let result = look_set.contains(Look::EndCRLF);
}

#[test]
fn test_contains_wordascii() {
    let look_set = LookSet { bits: Look::WordAscii.as_repr() };
    let result = look_set.contains(Look::WordAscii);
}

#[test]
fn test_contains_wordasciinegate() {
    let look_set = LookSet { bits: Look::WordAsciiNegate.as_repr() };
    let result = look_set.contains(Look::WordAsciiNegate);
}

#[test]
fn test_contains_wordunicode() {
    let look_set = LookSet { bits: Look::WordUnicode.as_repr() };
    let result = look_set.contains(Look::WordUnicode);
}

#[test]
fn test_contains_wordunicodenegate() {
    let look_set = LookSet { bits: Look::WordUnicodeNegate.as_repr() };
    let result = look_set.contains(Look::WordUnicodeNegate);
}

#[test]
fn test_contains_wordstartascii() {
    let look_set = LookSet { bits: Look::WordStartAscii.as_repr() };
    let result = look_set.contains(Look::WordStartAscii);
}

#[test]
fn test_contains_wordendascii() {
    let look_set = LookSet { bits: Look::WordEndAscii.as_repr() };
    let result = look_set.contains(Look::WordEndAscii);
}

#[test]
fn test_contains_wordstartunicode() {
    let look_set = LookSet { bits: Look::WordStartUnicode.as_repr() };
    let result = look_set.contains(Look::WordStartUnicode);
}

#[test]
fn test_contains_wordendunicode() {
    let look_set = LookSet { bits: Look::WordEndUnicode.as_repr() };
    let result = look_set.contains(Look::WordEndUnicode);
}

#[test]
fn test_contains_wordstarthalfascii() {
    let look_set = LookSet { bits: Look::WordStartHalfAscii.as_repr() };
    let result = look_set.contains(Look::WordStartHalfAscii);
}

#[test]
fn test_contains_wordendhalfascii() {
    let look_set = LookSet { bits: Look::WordEndHalfAscii.as_repr() };
    let result = look_set.contains(Look::WordEndHalfAscii);
}

#[test]
fn test_contains_wordstarthalfunicode() {
    let look_set = LookSet { bits: Look::WordStartHalfUnicode.as_repr() };
    let result = look_set.contains(Look::WordStartHalfUnicode);
}

#[test]
fn test_contains_wordendhalfunicode() {
    let look_set = LookSet { bits: Look::WordEndHalfUnicode.as_repr() };
    let result = look_set.contains(Look::WordEndHalfUnicode);
}

#[test]
fn test_contains_none() {
    let look_set = LookSet { bits: 0 };
    let result = look_set.contains(Look::Start);
}

