// Answer 0

#[test]
fn test_partitioning_case_10() {
    let o = Utf8Range { start: 0, end: 5 };
    let n = Utf8Range { start: 5, end: 10 };
    let result = Split::new(o, n);
    // Function result can be checked here if needed
}

#[test]
fn test_partitioning_case_13() {
    let o = Utf8Range { start: 3, end: 5 };
    let n = Utf8Range { start: 5, end: 10 };
    let result = Split::new(o, n);
    // Function result can be checked here if needed
}

#[test]
fn test_partitioning_case_12() {
    let o = Utf8Range { start: 4, end: 5 };
    let n = Utf8Range { start: 5, end: 10 };
    let result = Split::new(o, n);
    // Function result can be checked here if needed
}

