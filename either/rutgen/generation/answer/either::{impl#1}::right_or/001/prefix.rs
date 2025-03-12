// Answer 0

#[test]
fn test_right_or_with_integer() {
    let right: Either<i32, i32> = Right(42);
    let result = right.right_or(10);
}

#[test]
fn test_right_or_with_string() {
    let right: Either<&str, &str> = Right("hello");
    let result = right.right_or("world");
}

#[test]
fn test_right_or_with_user_defined_type() {
    struct CustomType {
        value: i32,
    }

    let right: Either<CustomType, CustomType> = Right(CustomType { value: 100 });
    let result = right.right_or(CustomType { value: 200 });
}

