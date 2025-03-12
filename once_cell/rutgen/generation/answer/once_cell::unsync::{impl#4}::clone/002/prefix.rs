// Answer 0

#[test]
fn test_clone_empty_once_cell() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    let cloned_cell = once_cell.clone();
}

#[test]
fn test_clone_empty_once_cell_with_string() {
    let once_cell: OnceCell<String> = OnceCell::new();
    let cloned_cell = once_cell.clone();
}

#[test]
fn test_clone_empty_once_cell_with_float() {
    let once_cell: OnceCell<f64> = OnceCell::new();
    let cloned_cell = once_cell.clone();
}

