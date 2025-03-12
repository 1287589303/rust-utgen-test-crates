// Answer 0

#[test]
fn test_split_case_when_y_equals_a() {
    let o = Utf8Range { start: 5, end: 10 };
    let n = Utf8Range { start: 10, end: 15 };
    let _result = Split::new(o, n);
}

#[test]
fn test_split_case_when_y_equals_a_and_x_less_than_b() {
    let o = Utf8Range { start: 5, end: 10 };
    let n = Utf8Range { start: 10, end: 12 };
    let _result = Split::new(o, n);
}

