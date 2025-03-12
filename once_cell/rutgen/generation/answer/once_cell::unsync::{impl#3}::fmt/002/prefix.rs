// Answer 0

#[test]
fn test_fmt_uninitialized_once_cell() {
    let cell: OnceCell<i32> = OnceCell::new();
    let formatter = &mut fmt::Formatter::new();
    let result = cell.fmt(formatter);
}

#[test]
fn test_fmt_unset_once_cell() {
    let cell: OnceCell<String> = OnceCell::new();
    let formatter = &mut fmt::Formatter::new();
    let result = cell.fmt(formatter);
}

#[test]
fn test_fmt_uninitialized_once_cell_float() {
    let cell: OnceCell<f64> = OnceCell::new();
    let formatter = &mut fmt::Formatter::new();
    let result = cell.fmt(formatter);
}

