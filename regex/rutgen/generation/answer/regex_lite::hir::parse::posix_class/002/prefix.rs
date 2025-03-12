// Answer 0

#[test]
fn test_posix_class_alpha() {
    let kind = "alpha";
    let result = posix_class(kind);
}

#[test]
#[should_panic]
fn test_posix_class_non_alnum() {
    let kind = "alnum"; // should not match, as per precondition
    let result = posix_class(kind);
}

