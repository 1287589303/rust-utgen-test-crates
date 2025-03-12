// Answer 0

#[test]
fn test_expect_right_with_integer() {
    let right: Either<(), i32> = Right(3);
    right.expect_right("value was Left");
}

#[test]
fn test_expect_right_with_string() {
    let right: Either<(), String> = Right(String::from("Hello"));
    right.expect_right("value was Left");
}

#[test]
fn test_expect_right_with_struct() {
    #[derive(Debug)]
    struct TestStruct {
        id: i32,
        name: String,
    }
    let right: Either<(), TestStruct> = Right(TestStruct { id: 1, name: String::from("Test") });
    right.expect_right("value was Left");
}

#[test]
#[should_panic(expected = "value was Right: 3")]
fn test_expect_right_panics_with_integer_left() {
    let left: Either<i32, ()> = Left(3);
    left.expect_right("value was Right");
}

#[test]
#[should_panic(expected = "value was Right: Hello")]
fn test_expect_right_panics_with_string_left() {
    let left: Either<String, ()> = Left(String::from("Hello"));
    left.expect_right("value was Right");
}

