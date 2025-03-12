// Answer 0

#[test]
fn test_split_new_case() {
    let o = Utf8Range { start: 5, end: 10 };
    let n = Utf8Range { start: 11, end: 15 };
    let _result = Split::new(o, n);
}

#[test]
fn test_split_new_case_boundary() {
    let o = Utf8Range { start: 10, end: 15 };
    let n = Utf8Range { start: 15, end: 20 };
    let _result = Split::new(o, n);
}

#[test]
fn test_split_new_case_reverse() {
    let o = Utf8Range { start: 1, end: 5 };
    let n = Utf8Range { start: 6, end: 10 };
    let _result = Split::new(o, n);
}

