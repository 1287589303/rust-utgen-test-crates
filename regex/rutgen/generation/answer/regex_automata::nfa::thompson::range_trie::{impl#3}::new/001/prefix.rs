// Answer 0

#[test]
fn test_new_case1() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 1, end: 1 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case2() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 2, end: 2 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case3() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 3, end: 5 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case4() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 4, end: 6 };
    let result = Split::new(o, n);
}

#[test]
fn test_new_case5() {
    let o = Utf8Range { start: 0, end: 0 };
    let n = Utf8Range { start: 5, end: 10 };
    let result = Split::new(o, n);
}

