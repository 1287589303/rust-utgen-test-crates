// Answer 0

#[test]
fn test_eq_both_uninitialized() {
    let cell1: OnceCell<i32> = OnceCell::new();
    let cell2: OnceCell<i32> = OnceCell::new();
    cell1.eq(&cell2);
}

#[test]
fn test_eq_first_uninitialized_second_initialized() {
    let cell1: OnceCell<i32> = OnceCell::new();
    let cell2 = OnceCell::with_value(42);
    cell1.eq(&cell2);
}

#[test]
fn test_eq_first_initialized_second_uninitialized() {
    let cell1 = OnceCell::with_value(42);
    let cell2: OnceCell<i32> = OnceCell::new();
    cell1.eq(&cell2);
}

#[test]
fn test_eq_both_initialized_equal() {
    let cell1 = OnceCell::with_value(42);
    let cell2 = OnceCell::with_value(42);
    cell1.eq(&cell2);
}

#[test]
fn test_eq_both_initialized_unequal() {
    let cell1 = OnceCell::with_value(42);
    let cell2 = OnceCell::with_value(43);
    cell1.eq(&cell2);
}

#[test]
fn test_eq_first_initialized_second_initialized_unequal() {
    let cell1 = OnceCell::with_value("Hello");
    let cell2 = OnceCell::with_value("World");
    cell1.eq(&cell2);
}

