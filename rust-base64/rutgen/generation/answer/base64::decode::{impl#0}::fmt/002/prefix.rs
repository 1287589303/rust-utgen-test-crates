// Answer 0

#[test]
fn test_invalid_last_symbol_zero_index() {
    let error = DecodeError::InvalidLastSymbol(0, 0);
    let mut f = fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_invalid_last_symbol_max_index() {
    let error = DecodeError::InvalidLastSymbol(usize::MAX, 255);
    let mut f = fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_invalid_last_symbol_middle_index() {
    let error = DecodeError::InvalidLastSymbol(123, 129);
    let mut f = fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_invalid_last_symbol_high_byte() {
    let error = DecodeError::InvalidLastSymbol(5, 250);
    let mut f = fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_invalid_last_symbol_low_byte() {
    let error = DecodeError::InvalidLastSymbol(2, 1);
    let mut f = fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

