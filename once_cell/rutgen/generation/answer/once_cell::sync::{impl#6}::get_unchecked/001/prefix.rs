// Answer 0

#[test]
fn test_get_unchecked_with_valid_value() {
    struct TestStruct;
    let once_cell = OnceCell::with_value(TestStruct);
    let reference: &TestStruct;

    unsafe {
        reference = once_cell.get_unchecked();
    }
}

#[test]
fn test_get_unchecked_with_initialized_int() {
    let once_cell = OnceCell::with_value(42);
    let reference: &i32;

    unsafe {
        reference = once_cell.get_unchecked();
    }
}

#[test]
fn test_get_unchecked_with_initialized_string() {
    let once_cell = OnceCell::with_value(String::from("Hello"));
    let reference: &String;

    unsafe {
        reference = once_cell.get_unchecked();
    }
}

