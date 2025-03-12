// Answer 0

#[test]
fn test_add_overflow_case() {
    let a = usize::MAX;
    let b = 1;
    let what = "Addition that exceeds usize max";
    let result = add(a, b, what);
}

#[test]
fn test_add_overflow_case_different_what() {
    let a = usize::MAX;
    let b = 1;
    let what = "Another addition exceeding max";
    let result = add(a, b, what);
}

