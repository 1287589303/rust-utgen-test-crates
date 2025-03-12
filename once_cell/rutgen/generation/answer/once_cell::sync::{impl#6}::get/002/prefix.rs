// Answer 0

#[test]
fn test_get_not_initialized() {
    let cell: OnceCell<i32> = OnceCell::new();
    let result = cell.get();
}

#[test]
fn test_get_with_default_value() {
    let cell: OnceCell<String> = OnceCell::new();
    let result = cell.get();
}

#[test]
fn test_get_empty_once_cell() {
    let cell: OnceCell<Vec<u8>> = OnceCell::new();
    let result = cell.get();
}

