// Answer 0

#[test]
fn test_new_empty_intersection() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 1, end: 0 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_boundary_b_equals_x() {
    let o = Utf8Range { start: 0, end: 1 };
    let n = Utf8Range { start: 1, end: 2 };
    let result = Split::new(o, n);
}

