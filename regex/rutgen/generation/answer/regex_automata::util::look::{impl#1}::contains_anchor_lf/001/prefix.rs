// Answer 0

#[test]
fn test_contains_anchor_lf_start_only() {
    let look_set = LookSet { bits: Look::StartLF as u32 };
    let _ = look_set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_end_only() {
    let look_set = LookSet { bits: Look::EndLF as u32 };
    let _ = look_set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_both() {
    let look_set = LookSet { bits: (Look::StartLF | Look::EndLF) as u32 };
    let _ = look_set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_none() {
    let look_set = LookSet { bits: 0 };
    let _ = look_set.contains_anchor_lf();
}

