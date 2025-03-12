// Answer 0

#[test]
fn test_eq_both_none() {
    let cell1: OnceCell<i32> = OnceCell::new();
    let cell2: OnceCell<i32> = OnceCell::new();
    cell1.eq(&cell2);
}

#[test]
fn test_eq_both_same_value() {
    let cell1 = OnceCell::with_value(42);
    let cell2 = OnceCell::with_value(42);
    cell1.eq(&cell2);
}

#[test]
fn test_eq_both_different_value() {
    let cell1 = OnceCell::with_value(42);
    let cell2 = OnceCell::with_value(43);
    cell1.eq(&cell2);
}

#[test]
fn test_eq_one_none_one_has_value() {
    let cell1: OnceCell<i32> = OnceCell::new();
    let cell2 = OnceCell::with_value(42);
    cell1.eq(&cell2);
}

#[test]
fn test_eq_one_has_value_one_none() {
    let cell1 = OnceCell::with_value(42);
    let cell2: OnceCell<i32> = OnceCell::new();
    cell1.eq(&cell2);
}

#[test]
fn test_eq_with_string_both_none() {
    let cell1: OnceCell<String> = OnceCell::new();
    let cell2: OnceCell<String> = OnceCell::new();
    cell1.eq(&cell2);
}

#[test]
fn test_eq_with_string_both_same_value() {
    let cell1 = OnceCell::with_value("Hello".to_string());
    let cell2 = OnceCell::with_value("Hello".to_string());
    cell1.eq(&cell2);
}

#[test]
fn test_eq_with_string_both_different_value() {
    let cell1 = OnceCell::with_value("Hello".to_string());
    let cell2 = OnceCell::with_value("World".to_string());
    cell1.eq(&cell2);
}

#[test]
fn test_eq_string_one_none_one_has_value() {
    let cell1: OnceCell<String> = OnceCell::new();
    let cell2 = OnceCell::with_value("Hello".to_string());
    cell1.eq(&cell2);
}

#[test]
fn test_eq_string_one_has_value_one_none() {
    let cell1 = OnceCell::with_value("Hello".to_string());
    let cell2: OnceCell<String> = OnceCell::new();
    cell1.eq(&cell2);
}

