// Answer 0

#[test]
fn test_cloned_right_non_empty() {
    let mut right_value = String::from("test");
    let either = Either::Right(&mut right_value);
    let result = either.cloned();
}

#[test]
fn test_cloned_right_empty() {
    let mut right_value = String::from("");
    let either = Either::Right(&mut right_value);
    let result = either.cloned();
}

#[test]
fn test_cloned_right_default() {
    let mut right_value: i32 = Default::default(); // Default is 0 for i32
    let either = Either::Right(&mut right_value);
    let result = either.cloned();
}

#[test]
fn test_cloned_right_cloneable_struct() {
    #[derive(Clone)]
    struct CustomStruct {
        value: i32,
    }
    let mut right_value = CustomStruct { value: 10 };
    let either = Either::Right(&mut right_value);
    let result = either.cloned();
}

