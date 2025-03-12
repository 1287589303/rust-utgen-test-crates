// Answer 0

#[test]
fn test_split_boundary_case() {
    let o = Utf8Range { start: 5, end: 5 };
    let n = Utf8Range { start: 6, end: 10 };
    Split::new(o, n);
}

