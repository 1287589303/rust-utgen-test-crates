// Answer 0

#[test]
fn test_contains_anchor_crlf_false_no_end() {
    let mut look_set = LookSet { bits: 0 };
    assert!(!look_set.contains_anchor_crlf());
}

#[test]
fn test_contains_anchor_crlf_false_with_only_start() {
    let look_set = LookSet { bits: Look::StartCRLF as u32 };
    assert!(!look_set.contains_anchor_crlf());
}

#[test]
fn test_contains_anchor_crlf_true_with_end() {
    let look_set = LookSet { bits: Look::EndCRLF as u32 };
    assert!(look_set.contains_anchor_crlf());
}

#[test]
fn test_contains_anchor_crlf_false_full() {
    let look_set = LookSet { bits: 0xFFFFFFFF };
    look_set.bits &= !(Look::StartCRLF as u32);
    assert!(!look_set.contains_anchor_crlf());
}

