// Answer 0

#[test]
fn test_new_case_4() {
    let o = Utf8Range { start: 1, end: 3 };
    let n = Utf8Range { start: 4, end: 6 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_4_boundary() {
    let o = Utf8Range { start: 1, end: 3 };
    let n = Utf8Range { start: 4, end: 5 }; // Boundary case
    let result = Split::new(o, n);
}

