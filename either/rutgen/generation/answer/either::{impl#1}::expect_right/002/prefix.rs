// Answer 0

#[test]
#[should_panic(expected = "value was Right: 3")]
fn test_expect_right_with_integer_debug() {
    let left: Either<i32, ()> = Left(3);
    left.expect_right("value was Right");
}

#[test]
#[should_panic(expected = "value was Right: test")]
fn test_expect_right_with_string_debug() {
    let left: Either<&str, ()> = Left("test");
    left.expect_right("value was Right");
}

#[test]
#[should_panic(expected = "value was Right: MyDebug")]
fn test_expect_right_with_custom_debug() {
    #[derive(Debug)]
    struct MyDebug;

    let left: Either<MyDebug, ()> = Left(MyDebug);
    left.expect_right("value was Right");
}

#[test]
#[should_panic(expected = "value was Right: (1, 2)")]
fn test_expect_right_with_tuple_debug() {
    let left: Either<(i32, i32), ()> = Left((1, 2));
    left.expect_right("value was Right");
}

