// Answer 0

#[test]
fn test_unwrap_right_panics_on_left() {
    let left: Either<i32, ()> = Left(3);
    left.unwrap_right();
}

#[test]
fn test_unwrap_right_panics_on_left_with_string() {
    let left: Either<String, ()> = Left(String::from("error"));
    left.unwrap_right();
}

#[test]
fn test_unwrap_right_panics_on_left_with_float() {
    let left: Either<f64, ()> = Left(3.14);
    left.unwrap_right();
}

#[test]
fn test_unwrap_right_panics_on_left_with_struct() {
    #[derive(Debug)]
    struct TestStruct {
        value: i32,
    }
    let left: Either<TestStruct, ()> = Left(TestStruct { value: 42 });
    left.unwrap_right();
}

#[test]
fn test_unwrap_right_panics_on_left_with_tuple() {
    let left: Either<(i32, i32), ()> = Left((1, 2));
    left.unwrap_right();
}

