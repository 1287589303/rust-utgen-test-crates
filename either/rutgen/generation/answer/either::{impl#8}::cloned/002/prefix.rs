// Answer 0

#[test]
fn test_cloned_left() {
    let value_left = String::from("left_value");
    let value_right = String::from("right_value");
    let either_instance = Either::Left(&value_left);
    let cloned_result = either_instance.cloned();
}

#[test]
fn test_cloned_right() {
    let value_left = String::from("left_value");
    let value_right = String::from("right_value");
    let either_instance = Either::Right(&value_right);
    let cloned_result = either_instance.cloned();
}

#[test]
fn test_cloned_left_empty_string() {
    let value_left = String::from("");
    let value_right = String::from("right_value");
    let either_instance = Either::Left(&value_left);
    let cloned_result = either_instance.cloned();
}

#[test]
fn test_cloned_right_empty_string() {
    let value_left = String::from("left_value");
    let value_right = String::from("");
    let either_instance = Either::Right(&value_right);
    let cloned_result = either_instance.cloned();
}

#[test]
fn test_cloned_left_numeric() {
    let value_left: i32 = 42;
    let value_right: i32 = 84;
    let either_instance = Either::Left(&value_left);
    let cloned_result = either_instance.cloned();
}

#[test]
fn test_cloned_right_numeric() {
    let value_left: i32 = 42;
    let value_right: i32 = 84;
    let either_instance = Either::Right(&value_right);
    let cloned_result = either_instance.cloned();
}

