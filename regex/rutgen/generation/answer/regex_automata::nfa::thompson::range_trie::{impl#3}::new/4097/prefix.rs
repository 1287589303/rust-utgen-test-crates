// Answer 0

#[test]
fn test_new_case_3() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 0, end: 0 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_4() {
    let o = Utf8Range { start: 0, end: 1 };
    let n = Utf8Range { start: 1, end: 1 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_5() {
    let o = Utf8Range { start: 1, end: 1 };
    let n = Utf8Range { start: 0, end: 1 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_6() {
    let o = Utf8Range { start: 0, end: 1 };
    let n = Utf8Range { start: 0, end: 0 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_7() {
    let o = Utf8Range { start: 1, end: 1 };
    let n = Utf8Range { start: 1, end: 1 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_8() {
    let o = Utf8Range { start: 1, end: 2 };
    let n = Utf8Range { start: 0, end: 1 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_9() {
    let o = Utf8Range { start: 0, end: 1 };
    let n = Utf8Range { start: 1, end: 2 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_10() {
    let o = Utf8Range { start: 0, end: 1 };
    let n = Utf8Range { start: 1, end: 1 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_11() {
    let o = Utf8Range { start: 1, end: 2 };
    let n = Utf8Range { start: 2, end: 3 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_12() {
    let o = Utf8Range { start: 0, end: 1 };
    let n = Utf8Range { start: 2, end: 3 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_13() {
    let o = Utf8Range { start: 2, end: 3 };
    let n = Utf8Range { start: 1, end: 2 };
    let result = Split::new(o, n);
}

