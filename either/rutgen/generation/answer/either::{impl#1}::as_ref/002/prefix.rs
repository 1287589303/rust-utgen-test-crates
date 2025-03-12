// Answer 0

#[test]
fn test_as_ref_left_variant() {
    let left: Either<String, ()> = Left(String::from("some value"));
    let result = left.as_ref();
}

#[test]
fn test_as_ref_left_variant_empty_string() {
    let left: Either<String, ()> = Left(String::from(""));
    let result = left.as_ref();
}

#[test]
fn test_as_ref_left_variant_large_string() {
    let left: Either<String, ()> = Left(String::from("a".repeat(1000)));
    let result = left.as_ref();
}

