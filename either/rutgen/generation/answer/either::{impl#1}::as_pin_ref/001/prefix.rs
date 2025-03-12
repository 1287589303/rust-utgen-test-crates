// Answer 0

#[test]
fn test_as_pin_ref_with_right_variant() {
    struct TestStruct;

    let value = Right(TestStruct);
    let pinned_value = Pin::new(Box::new(value));

    let _result: Either<Pin<&TestStruct>, Pin<&()>> = pinned_value.as_pin_ref();
}

#[test]
fn test_as_pin_ref_with_different_right_variant() {
    struct AnotherStruct;

    let value = Right(AnotherStruct);
    let pinned_value = Pin::new(Box::new(value));

    let _result: Either<Pin<&AnotherStruct>, Pin<&()>> = pinned_value.as_pin_ref();
}

#[test]
fn test_as_pin_ref_with_unpin_right_variant() {
    struct UnpinStruct;

    let value = Right(UnpinStruct);
    let pinned_value = Pin::new(Box::new(value));

    let _result: Either<Pin<&UnpinStruct>, Pin<&()>> = pinned_value.as_pin_ref();
}

