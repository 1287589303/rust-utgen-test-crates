// Answer 0

#[test]
fn test_set_with_existing_value_i32() {
    let cell = OnceCell::with_value(1);
    let result = cell.set(2);
    let _ = result; // Using the result, we're testing the return value but not asserting
}

#[test]
fn test_set_with_existing_value_f32() {
    let cell = OnceCell::with_value(2.5);
    let result = cell.set(3.0);
    let _ = result; // Using the result, we're testing the return value but not asserting
}

#[test]
fn test_set_with_existing_value_string() {
    let cell = OnceCell::with_value(String::from("Hello"));
    let result = cell.set(String::from("World"));
    let _ = result; // Using the result, we're testing the return value but not asserting
}

#[test]
fn test_set_with_existing_value_char() {
    let cell = OnceCell::with_value('a');
    let result = cell.set('b');
    let _ = result; // Using the result, we're testing the return value but not asserting
}

