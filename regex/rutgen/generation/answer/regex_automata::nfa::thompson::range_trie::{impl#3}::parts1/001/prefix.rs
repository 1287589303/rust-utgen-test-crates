// Answer 0

#[test]
fn test_parts1_valid_range() {
    let r1 = SplitRange::Old(Utf8Range { start: 1, end: 2 });
    let split = Split::parts1(r1);
}

#[test]
fn test_parts1_edge_case() {
    let r1 = SplitRange::Old(Utf8Range { start: 2, end: 2 });
    let split = Split::parts1(r1);
}

#[test]
#[should_panic]
fn test_parts1_invalid_range() {
    let r1 = SplitRange::Old(Utf8Range { start: 3, end: 2 });
    let split = Split::parts1(r1);
}

