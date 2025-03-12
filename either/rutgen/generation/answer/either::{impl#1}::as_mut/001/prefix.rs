// Answer 0

#[test]
fn test_as_mut_with_right_variant() {
    struct TestStruct {
        value: u32,
    }

    let mut right = Right(TestStruct { value: 123 });
    let result: Either<&mut TestStruct, &mut ()> = right.as_mut();
    let right_value: &mut TestStruct = result.right().unwrap();
    right_value.value = 999;
}

#[test]
fn test_as_mut_with_right_variant_empty_struct() {
    struct EmptyStruct;

    let mut right = Right(EmptyStruct);
    let result: Either<&mut EmptyStruct, &mut ()> = right.as_mut();
    let _right_value: &mut EmptyStruct = result.right().unwrap();
}

#[test]
fn test_as_mut_with_right_variant_string() {
    let mut right = Right(String::from("initial"));
    let result: Either<&mut String, &mut ()> = right.as_mut();
    let right_value: &mut String = result.right().unwrap();
    right_value.push_str(" updated");
}

