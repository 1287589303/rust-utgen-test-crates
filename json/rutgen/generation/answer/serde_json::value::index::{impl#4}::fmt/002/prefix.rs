// Answer 0

#[test]
fn test_fmt_array_empty() {
    let value = Value::Array(Vec::new());
    let typ = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = typ.fmt(&mut formatter);
}

#[test]
fn test_fmt_array_single_element() {
    let value = Value::Array(vec![Value::Bool(true)]);
    let typ = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = typ.fmt(&mut formatter);
}

#[test]
fn test_fmt_array_multiple_elements() {
    let value = Value::Array(vec![Value::Number(Number::from(42)), Value::String("test".to_string())]);
    let typ = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = typ.fmt(&mut formatter);
}

#[test]
fn test_fmt_array_with_null() {
    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);
    let typ = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = typ.fmt(&mut formatter);
}

