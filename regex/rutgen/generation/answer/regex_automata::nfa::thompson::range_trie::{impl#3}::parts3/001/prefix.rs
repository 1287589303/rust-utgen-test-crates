// Answer 0

#[test]
fn test_parts3_with_old_new_both() {
    let old_range = Utf8Range::new(0, 1).unwrap();
    let new_range = Utf8Range::new(2, 3).unwrap();
    let both_range = Utf8Range::new(4, 5).unwrap();
    let r1 = SplitRange::Old(old_range);
    let r2 = SplitRange::New(new_range);
    let r3 = SplitRange::Both(both_range);
    parts3(r1, r2, r3);
}

#[test]
fn test_parts3_with_empty_utf8_ranges() {
    let old_range = Utf8Range::new(0, 0).unwrap();
    let new_range = Utf8Range::new(0, 0).unwrap();
    let both_range = Utf8Range::new(0, 0).unwrap();
    let r1 = SplitRange::Old(old_range);
    let r2 = SplitRange::New(new_range);
    let r3 = SplitRange::Both(both_range);
    parts3(r1, r2, r3);
}

#[test]
fn test_parts3_with_full_utf8_character_set() {
    let old_range = Utf8Range::new(0, 0x10FFFF).unwrap();
    let new_range = Utf8Range::new(0, 0x10FFFF).unwrap();
    let both_range = Utf8Range::new(0, 0x10FFFF).unwrap();
    let r1 = SplitRange::Old(old_range);
    let r2 = SplitRange::New(new_range);
    let r3 = SplitRange::Both(both_range);
    parts3(r1, r2, r3);
}

