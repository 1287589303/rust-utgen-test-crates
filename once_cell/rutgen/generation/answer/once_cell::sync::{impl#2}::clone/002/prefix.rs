// Answer 0

#[test]
fn test_clone_none() {
    let cell: OnceCell<i32> = OnceCell::new();
    let cloned_cell = cell.clone();
}

#[test]
fn test_clone_none_with_string() {
    let cell: OnceCell<String> = OnceCell::new();
    let cloned_cell = cell.clone();
}

#[test]
fn test_clone_none_with_struct() {
    struct TestStruct {
        value: i32,
    }
    let cell: OnceCell<TestStruct> = OnceCell::new();
    let cloned_cell = cell.clone();
}

