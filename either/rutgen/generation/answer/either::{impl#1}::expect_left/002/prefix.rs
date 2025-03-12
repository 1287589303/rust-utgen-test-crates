// Answer 0

#[test]
fn test_expect_left_with_integer() {
    let left: Either<i32, ()> = Left(42);
    let _result = left.expect_left("value was Right");
}

#[test]
fn test_expect_left_with_string() {
    let left: Either<&str, ()> = Left("Hello");
    let _result = left.expect_left("value was Right");
}

#[test]
fn test_expect_left_with_floating_point() {
    let left: Either<f64, ()> = Left(3.14);
    let _result = left.expect_left("value was Right");
}

#[test]
fn test_expect_left_with_struct() {
    #[derive(Debug)]
    struct MyStruct {
        value: i32,
    }
    let left: Either<MyStruct, ()> = Left(MyStruct { value: 10 });
    let _result = left.expect_left("value was Right");
}

#[test]
fn test_expect_left_with_empty_tuple() {
    let left: Either<(), ()> = Left(());
    let _result = left.expect_left("value was Right");
}

