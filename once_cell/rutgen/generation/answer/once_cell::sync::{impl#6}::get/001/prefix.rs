// Answer 0

#[test]
fn test_get_with_initialized_integer() {
    let cell = OnceCell::with_value(42);
    let result = cell.get();
}

#[test]
fn test_get_with_initialized_string() {
    let cell = OnceCell::with_value(String::from("Hello"));
    let result = cell.get();
}

#[test]
fn test_get_with_initialized_struct() {
    struct TestStruct {
        field: i32,
    }

    let cell = OnceCell::with_value(TestStruct { field: 10 });
    let result = cell.get();
}

