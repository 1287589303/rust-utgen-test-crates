// Answer 0

#[test]
fn test_new_case_1() {
    let o = Utf8Range { start: 5, end: 5 };
    let n = Utf8Range { start: 5, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_2() {
    let o = Utf8Range { start: 8, end: 8 };
    let n = Utf8Range { start: 5, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_3() {
    let o = Utf8Range { start: 5, end: 5 };
    let n = Utf8Range { start: 8, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_4() {
    let o = Utf8Range { start: 5, end: 7 };
    let n = Utf8Range { start: 8, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_5() {
    let o = Utf8Range { start: 5, end: 7 };
    let n = Utf8Range { start: 7, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_6() {
    let o = Utf8Range { start: 5, end: 8 };
    let n = Utf8Range { start: 8, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case_7() {
    let o = Utf8Range { start: 8, end: 8 };
    let n = Utf8Range { start: 5, end: 8 };
    let result = Split::new(o, n);
}

