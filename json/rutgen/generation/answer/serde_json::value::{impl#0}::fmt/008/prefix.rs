// Answer 0

#[test]
fn test_fmt_null() {
    let value = Value::Null;
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_bool() {
    let value = Value::Bool(true);
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_number() {
    let number = Number { n: 12 };
    let value = Value::Number(number);
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_string() {
    let value = Value::String(String::from("a string"));
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_array() {
    let value = Value::Array(Vec::new());
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

#[test]
fn test_fmt_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

