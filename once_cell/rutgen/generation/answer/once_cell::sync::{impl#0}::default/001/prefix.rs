// Answer 0

#[test]
fn test_once_cell_default_with_i32() {
    let cell: OnceCell<i32> = OnceCell::default();
    let _ = cell.get();
}

#[test]
fn test_once_cell_default_with_bool() {
    let cell: OnceCell<bool> = OnceCell::default();
    let _ = cell.get();
}

#[test]
fn test_once_cell_default_with_str_reference() {
    let cell: OnceCell<&str> = OnceCell::default();
    let _ = cell.get();
}

#[test]
fn test_once_cell_default_with_custom_struct() {
    struct CustomStruct {
        value: i32,
    }

    let cell: OnceCell<CustomStruct> = OnceCell::default();
    let _ = cell.get();
}

