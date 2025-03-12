// Answer 0

#[test]
fn test_contains_anchor_lf_include_start() {
    let mut look_set = LookSet { bits: 0b00000100 }; // contains Look::StartLF
    let _ = look_set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_include_start_and_end() {
    let mut look_set = LookSet { bits: 0b00000110 }; // contains Look::StartLF and Look::EndLF
    let _ = look_set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_include_end_only() {
    let mut look_set = LookSet { bits: 0b00000000 }; // does not contain Look::StartLF, should not be valid
    look_set.set_insert(Look::EndLF); // contains Look::EndLF
    let _ = look_set.contains_anchor_lf();
}

