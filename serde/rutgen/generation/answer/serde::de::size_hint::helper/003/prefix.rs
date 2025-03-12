// Answer 0

#[test]
fn test_helper_lower_not_equal_upper_with_some() {
    let bounds = (5, Some(10)); 
    let _ = helper(bounds);
}

#[test]
fn test_helper_lower_not_equal_upper_with_none() {
    let bounds = (3, None); 
    let _ = helper(bounds);
}

#[test]
fn test_helper_lower_equal_upper_with_some() {
    let bounds = (7, Some(7)); 
    let _ = helper(bounds);
}

