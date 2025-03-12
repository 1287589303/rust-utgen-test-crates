// Answer 0

#[test]
fn test_once_cell_uninitialized_fmt() {
    let once_cell: OnceCell<i32> = OnceCell::new();
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_uninitialized_fmt_string() {
    let once_cell: OnceCell<String> = OnceCell::new();
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_uninitialized_fmt_tuple() {
    let once_cell: OnceCell<(i32, i32)> = OnceCell::new();
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_uninitialized_fmt_float() {
    let once_cell: OnceCell<f64> = OnceCell::new();
    let mut formatter = fmt::Formatter::new();
    let _ = once_cell.fmt(&mut formatter);
}

