// Answer 0

#[test]
fn test_get_empty_once_cell() {
    let cell: OnceCell<i32> = OnceCell::new();
    let result = cell.get();
}

#[test]
fn test_get_with_value() {
    let cell = OnceCell::with_value(42);
    let result = cell.get();
}

#[test]
fn test_get_after_set() {
    let cell: OnceCell<i32> = OnceCell::new();
    let _ = cell.set(42);
    let result = cell.get();
}

#[test]
fn test_get_multiple_calls() {
    let cell: OnceCell<i32> = OnceCell::new();
    let result_first_call = cell.get();
    let _ = cell.set(42);
    let result_second_call = cell.get();
    let result_third_call = cell.get();
}

