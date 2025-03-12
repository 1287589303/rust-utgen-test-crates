// Answer 0

#[test]
fn test_get_or_init_with_valid_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    let _ = value; // Use the value to ensure it's being called
}

#[test]
fn test_get_or_init_with_zero_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| 0);
    let _ = value; // Use the value to ensure it's being called
}

#[test]
fn test_get_or_init_with_negative_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| -1);
    let _ = value; // Use the value to ensure it's being called
}

#[test]
fn test_get_or_init_with_large_value() {
    let cell: OnceCell<u64> = OnceCell::new();
    let value = cell.get_or_init(|| 1_000_000_000);
    let _ = value; // Use the value to ensure it's being called
}

#[test]
fn test_get_or_init_with_string() {
    let cell: OnceCell<String> = OnceCell::new();
    let value = cell.get_or_init(|| String::from("Hello, world!"));
    let _ = value; // Use the value to ensure it's being called
}

