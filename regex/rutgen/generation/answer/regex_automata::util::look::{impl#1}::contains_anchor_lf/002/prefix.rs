// Answer 0

#[test]
fn test_contains_anchor_lf_empty() {
    let set = LookSet::empty();
    set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_only_endlf() {
    let set = LookSet { bits: 0b0000_0010 };
    set.contains_anchor_lf();
}

#[test]
fn test_contains_anchor_lf_only_startlf() {
    let set = LookSet { bits: 0b0000_0001 };
    set.contains_anchor_lf();
}

