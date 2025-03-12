// Answer 0

#[test]
fn test_helper_lower_none() {
    let bounds = (0, None);
    let result = helper(bounds);
}

#[test]
fn test_helper_lower_non_matching_upper() {
    let bounds = (1, Some(2));
    let result = helper(bounds);
}

#[test]
fn test_helper_lower_non_matching_upper_large() {
    let bounds = (usize::MAX, Some(usize::MAX - 1));
    let result = helper(bounds);
}

#[test]
fn test_helper_lower_zero_non_matching_upper() {
    let bounds = (0, Some(1));
    let result = helper(bounds);
}

#[test]
fn test_helper_lower_min_non_matching_upper() {
    let bounds = (usize::MIN, Some(1));
    let result = helper(bounds);
}

