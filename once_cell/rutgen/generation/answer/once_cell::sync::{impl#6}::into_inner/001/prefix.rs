// Answer 0

#[test]
fn test_into_inner_empty() {
    let cell: OnceCell<String> = OnceCell::new();
    let result = cell.into_inner();
}

#[test]
fn test_into_inner_with_value() {
    let cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    let result = cell.into_inner();
}

#[test]
fn test_into_inner_with_different_value() {
    let cell = OnceCell::new();
    cell.set("world".to_string()).unwrap();
    let result = cell.into_inner();
}

#[test]
fn test_into_inner_double_insert() {
    let cell = OnceCell::new();
    cell.set("first".to_string()).unwrap();
    let _ = cell.set("second".to_string()); // This should not panic
    let result = cell.into_inner();
}

