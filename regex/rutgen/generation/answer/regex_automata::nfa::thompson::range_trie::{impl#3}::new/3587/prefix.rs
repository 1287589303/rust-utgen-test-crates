// Answer 0

#[test]
fn test_new_case_12() {
    let o = Utf8Range { start: 1, end: 3 };
    let n = Utf8Range { start: 3, end: 5 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_11() {
    let o = Utf8Range { start: 2, end: 4 };
    let n = Utf8Range { start: 3, end: 5 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_10() {
    let o = Utf8Range { start: 1, end: 5 };
    let n = Utf8Range { start: 5, end: 5 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_9() {
    let o = Utf8Range { start: 2, end: 6 };
    let n = Utf8Range { start: 4, end: 5 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_8() {
    let o = Utf8Range { start: 2, end: 3 };
    let n = Utf8Range { start: 3, end: 6 };
    let result = Split::new(o, n);
}

