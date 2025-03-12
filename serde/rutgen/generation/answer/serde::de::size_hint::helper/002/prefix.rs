// Answer 0

#[test]
fn test_helper_with_zero() {
    let bounds = (0, Some(0));
    let result = helper(bounds);
}

#[test]
fn test_helper_with_one() {
    let bounds = (1, Some(1));
    let result = helper(bounds);
}

#[test]
fn test_helper_with_hundred() {
    let bounds = (100, Some(100));
    let result = helper(bounds);
}

