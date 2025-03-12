// Answer 0

#[test]
fn test_split_new_case_8() {
    let o = Utf8Range { start: 10, end: 15 };
    let n = Utf8Range { start: 16, end: 20 };
    Split::new(o, n);
}

#[test]
fn test_split_new_case_8_boundary() {
    let o = Utf8Range { start: 10, end: 15 };
    let n = Utf8Range { start: 15, end: 20 };
    Split::new(o, n);
}

#[test]
fn test_split_new_case_8_another_boundary() {
    let o = Utf8Range { start: 10, end: 15 };
    let n = Utf8Range { start: 14, end: 20 };
    Split::new(o, n);
}

