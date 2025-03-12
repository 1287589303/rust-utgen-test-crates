// Answer 0

#[test]
fn test_contains_anchor_line_start_lf_only() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::StartLF);
    let _ = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_start_lf_and_end_lf() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndLF);
    let _ = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_start_lf_and_start_crlf() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::StartCRLF);
    let _ = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_start_lf_and_end_crlf() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndCRLF);
    let _ = look_set.contains_anchor_line();
}

#[test]
fn test_contains_anchor_line_all_line_anchors() {
    let mut look_set = LookSet { bits: 0 };
    look_set.set_insert(Look::StartLF);
    look_set.set_insert(Look::EndLF);
    look_set.set_insert(Look::StartCRLF);
    look_set.set_insert(Look::EndCRLF);
    let _ = look_set.contains_anchor_line();
}

