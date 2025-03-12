// Answer 0

#[test]
fn test_fmt_requested_greater_than_available() {
    let error = TryGetError {
        requested: 10,
        available: 5,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_requested_equals_available() {
    let error = TryGetError {
        requested: 5,
        available: 5,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_requested_one_available_zero() {
    let error = TryGetError {
        requested: 1,
        available: 0,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_requested_ten_available_zero() {
    let error = TryGetError {
        requested: 10,
        available: 0,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_requested_ten_available_five() {
    let error = TryGetError {
        requested: 10,
        available: 5,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_requested_two_available_one() {
    let error = TryGetError {
        requested: 2,
        available: 1,
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

