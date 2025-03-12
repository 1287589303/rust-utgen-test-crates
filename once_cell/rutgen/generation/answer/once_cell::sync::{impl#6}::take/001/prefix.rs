// Answer 0

#[test]
fn test_once_cell_take_uninitialized_string() {
    let mut cell: OnceCell<String> = OnceCell::new();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_initialized_string() {
    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_empty_string() {
    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("".to_string()).unwrap();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_uninitialized_u32() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_initialized_u32() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_zero_u32() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(0).unwrap();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_negative_i32() {
    let mut cell: OnceCell<i32> = OnceCell::new();
    cell.set(-1).unwrap();
    let result = cell.take();
}

#[test]
fn test_once_cell_take_positive_i32() {
    let mut cell: OnceCell<i32> = OnceCell::new();
    cell.set(10).unwrap();
    let result = cell.take();
}

