// Answer 0

#[test]
fn test_new_case_3() {
    let o = Utf8Range { start: 1, end: 1 };
    let n = Utf8Range { start: 1, end: 1 };
    Split::new(o, n);
}

#[test]
fn test_new_edge_case_equal() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 0, end: 0 };
    Split::new(o, n);
}

#[test]
fn test_new_case_3_both_zero() {
    let o = Utf8Range { start: 1, end: 1 };
    let n = Utf8Range { start: 1, end: 1 };
    Split::new(o, n);
}

