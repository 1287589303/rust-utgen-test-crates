// Answer 0

#[test]
fn test_contains_anchor_crlf_empty() {
    let set = LookSet::empty();
    set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_only_endcrlf() {
    let mut set = LookSet::default();
    set.set_insert(Look::EndCRLF);
    set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_multiple_bits() {
    let mut set = LookSet::default();
    set.set_insert(Look::EndCRLF);
    set.set_insert(Look::WordAscii);
    set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_multiple_bits_excluding_startcrlf() {
    let mut set = LookSet::default();
    set.set_insert(Look::EndCRLF);
    set.set_insert(Look::StartLF);
    set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_no_endcrlf() {
    let mut set = LookSet::default();
    set.set_insert(Look::WordStartAscii);
    set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_with_other_bits() {
    let mut set = LookSet::default();
    set.set_insert(Look::Start);
    set.set_insert(Look::End);
    set.contains_anchor_crlf();
}

#[test]
fn test_contains_anchor_crlf_full_set_except_startcrlf() {
    let mut set = LookSet::default();
    set.set_insert(Look::EndCRLF);
    set.set_insert(Look::WordEndAscii);
    set.set_insert(Look::WordStartHalfAscii);
    set.set_insert(Look::WordEndHalfAscii);
    set.contains_anchor_crlf();
}

