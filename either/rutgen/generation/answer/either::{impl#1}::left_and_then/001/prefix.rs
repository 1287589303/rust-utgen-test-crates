// Answer 0

#[test]
fn test_left_and_then_with_right_variant() {
    let right: Either<u32, _> = Right(123);
    let _result = right.left_and_then(|x| Right(x * 2));
}

#[test]
fn test_left_and_then_with_right_variant_string() {
    let right: Either<(), String> = Right(String::from("Test"));
    let _result = right.left_and_then(|x| Right(x.len().to_string()));
}

#[test]
fn test_left_and_then_with_right_variant_custom_type() {
    struct CustomType {
        value: i32,
    }

    let right: Either<CustomType, _> = Right(CustomType { value: 42 });
    let _result = right.left_and_then(|_x| Right("Still Right"));
}

#[test]
fn test_left_and_then_with_right_variant_empty() {
    let right: Either<(), ()> = Right(());
    let _result = right.left_and_then(|_x| Right("Not used"));
}

