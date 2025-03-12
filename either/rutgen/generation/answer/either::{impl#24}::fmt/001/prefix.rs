// Answer 0

#[test]
fn test_fmt_with_right_integer_zero() {
    let right_value = 0;
    let either_value = Right(right_value);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", either_value);
}

#[test]
fn test_fmt_with_right_integer_negative() {
    let right_value = -1;
    let either_value = Right(right_value);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", either_value);
}

#[test]
fn test_fmt_with_right_float() {
    let right_value = 1.0;
    let either_value = Right(right_value);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", either_value);
}

#[test]
fn test_fmt_with_right_string() {
    let right_value = "string";
    let either_value = Right(right_value);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", either_value);
}

#[test]
#[should_panic]
fn test_fmt_with_right_non_displayable() {
    struct NonDisplayable;
    let either_value = Right(NonDisplayable);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", either_value);
}

