// Answer 0

#[test]
fn test_split_new_case7() {
    let o = Utf8Range { start: 2, end: 5 };
    let n = Utf8Range { start: 6, end: 6 };
    Split::new(o, n);
}

#[test]
fn test_split_new_case12() {
    let o = Utf8Range { start: 3, end: 5 };
    let n = Utf8Range { start: 5, end: 6 };
    Split::new(o, n);
}

