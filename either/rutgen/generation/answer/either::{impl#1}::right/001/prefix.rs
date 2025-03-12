// Answer 0

#[test]
fn test_right_with_integer() {
    let right: Either<(), i32> = Right(123);
    right.right();
}

#[test]
fn test_right_with_string() {
    let right: Either<(), String> = Right(String::from("Hello"));
    right.right();
}

#[test]
fn test_right_with_custom_struct() {
    struct CustomStruct {
        value: i32,
    }
    let right: Either<(), CustomStruct> = Right(CustomStruct { value: 42 });
    right.right();
}

