// Answer 0

#[test]
fn test_parts2_edge_case_zero_zero() {
    let r1 = SplitRange::Old(Utf8Range { start: 0, end: 0 });
    let r2 = SplitRange::New(Utf8Range { start: 0, end: 1 });
    let _result = Split::parts2(r1, r2);
}

#[test]
fn test_parts2_zero_one() {
    let r1 = SplitRange::New(Utf8Range { start: 0, end: 1 });
    let r2 = SplitRange::Both(Utf8Range { start: 0, end: 2 });
    let _result = Split::parts2(r1, r2);
}

#[test]
fn test_parts2_one_one() {
    let r1 = SplitRange::Both(Utf8Range { start: 1, end: 1 });
    let r2 = SplitRange::Old(Utf8Range { start: 0, end: 1 });
    let _result = Split::parts2(r1, r2);
}

#[test]
fn test_parts2_one_two() {
    let r1 = SplitRange::New(Utf8Range { start: 1, end: 2 });
    let r2 = SplitRange::Both(Utf8Range { start: 2, end: 3 });
    let _result = Split::parts2(r1, r2);
}

#[test]
fn test_parts2_both() {
    let r1 = SplitRange::Both(Utf8Range { start: 1, end: 3 });
    let r2 = SplitRange::Both(Utf8Range { start: 3, end: 4 });
    let _result = Split::parts2(r1, r2);
}

