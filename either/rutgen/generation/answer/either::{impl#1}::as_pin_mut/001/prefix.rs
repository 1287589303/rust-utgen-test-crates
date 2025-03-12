// Answer 0

#[test]
fn test_as_pin_mut_right_variant() {
    use std::pin::Pin;

    struct RightTestStruct {
        value: i32,
    }
    
    let mut right_value = RightTestStruct { value: 42 };
    let mut either_instance = Either::Right(right_value);
    
    let mut pinned_either = Pin::new(&mut either_instance);
    let result = pinned_either.as_pin_mut();
}

#[test]
fn test_as_pin_mut_right_variant_with_other_value() {
    use std::pin::Pin;

    struct AnotherRightTestStruct {
        value: String,
    }
    
    let mut right_value = AnotherRightTestStruct { value: String::from("Test") };
    let mut either_instance = Either::Right(right_value);
    
    let mut pinned_either = Pin::new(&mut either_instance);
    let result = pinned_either.as_pin_mut();
}


