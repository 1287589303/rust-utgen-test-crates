// Answer 0

#[test]
fn test_split_case_b_eq_x_and_a_gt_x() {
    let o = Utf8Range { start: 5, end: 7 };
    let n = Utf8Range { start: 7, end: 7 };
    let result = Split::new(o, n);
}

#[test]
fn test_split_case_b_eq_y_and_a_gt_x() {
    let o = Utf8Range { start: 6, end: 8 };
    let n = Utf8Range { start: 8, end: 8 };
    let result = Split::new(o, n);
}

#[test]
fn test_split_case_b_eq_y_and_a_gt_x_alternative() {
    let o = Utf8Range { start: 4, end: 6 };
    let n = Utf8Range { start: 6, end: 6 };
    let result = Split::new(o, n);
}

