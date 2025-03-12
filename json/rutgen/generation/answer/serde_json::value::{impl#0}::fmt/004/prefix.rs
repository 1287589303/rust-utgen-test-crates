// Answer 0

#[test]
fn test_fmt_array_empty() {
    let formatter = &mut fmt::Formatter::new();
    let value = Value::Array(Vec::new());
    value.fmt(formatter);
}

#[test]
fn test_fmt_array_single_element() {
    let formatter = &mut fmt::Formatter::new();
    let value = Value::Array(vec![Value::Bool(true)]);
    value.fmt(formatter);
}

#[test]
fn test_fmt_array_multiple_elements() {
    let formatter = &mut fmt::Formatter::new();
    let value = Value::Array(vec![
        Value::Bool(true),
        Value::Number(Number { n: 1 }),
        Value::String(String::from("example")),
        Value::Null,
    ]);
    value.fmt(formatter);
}

#[test]
fn test_fmt_array_boundary_conditions() {
    let formatter = &mut fmt::Formatter::new();
    let value_empty = Value::Array(Vec::new());
    value_empty.fmt(formatter);
    
    let value_max = Value::Array((0..1000).map(|_| Value::Number(Number { n: 0 })).collect());
    value_max.fmt(formatter);
}

