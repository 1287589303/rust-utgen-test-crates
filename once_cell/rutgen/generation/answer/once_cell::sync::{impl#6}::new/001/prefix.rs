// Answer 0

#[test]
fn test_once_cell_new() {
    struct TestStruct;
    let cell: OnceCell<TestStruct> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_integer() {
    let cell: OnceCell<i32> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_float() {
    let cell: OnceCell<f64> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_string() {
    let cell: OnceCell<String> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_tuple() {
    let cell: OnceCell<(i32, f64)> = OnceCell::new();
}

#[test]
fn test_once_cell_new_with_array() {
    let cell: OnceCell<[i32; 3]> = OnceCell::new();
}

