// Answer 0

#[test]
fn test_after_percent_sign_empty_iter() {
    let mut iter = [].iter();
    let result = after_percent_sign(&mut iter);
}

#[test]
fn test_after_percent_sign_one_element_iter() {
    let mut iter = [b'A'].iter();
    let result = after_percent_sign(&mut iter);
}

