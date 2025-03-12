// Answer 0

#[test]
fn test_once_cell_set_empty_cell() {
    struct TestCell(i32);
    let cell = OnceCell::new();
    let result = cell.set(TestCell(42));
}

#[test]
fn test_once_cell_set_non_empty_cell() {
    struct TestCell(i32);
    let cell = OnceCell::new();
    let _ = cell.set(TestCell(42));
    let result = cell.set(TestCell(84));
}

#[test]
fn test_once_cell_set_zero_value() {
    struct TestCell(i32);
    let cell = OnceCell::new();
    let result = cell.set(TestCell(0));
}

#[test]
fn test_once_cell_set_negative_value() {
    struct TestCell(i32);
    let cell = OnceCell::new();
    let result = cell.set(TestCell(-1));
}

#[test]
fn test_once_cell_set_maximum_value() {
    struct TestCell(i32);
    let cell = OnceCell::new();
    let result = cell.set(TestCell(i32::MAX));
}

#[test]
fn test_once_cell_set_special_value() {
    struct TestCell(String);
    let cell = OnceCell::new();
    let result = cell.set(TestCell(String::from("test")));
}

