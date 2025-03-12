// Answer 0

#[test]
fn test_split_case_9() {
    let o = Utf8Range { start: 5, end: 5 };
    let n = Utf8Range { start: 10, end: 15 };
    let result = Split::new(o, n);
}

#[test]
fn test_split_case_9_boundary() {
    let o = Utf8Range { start: 5, end: 5 };
    let n = Utf8Range { start: 10, end: 10 };
    let result = Split::new(o, n);
}

