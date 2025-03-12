// Answer 0

#[test]
fn test_display_object_with_non_empty_keys() {
    use crate::map::Map;
    
    let object = Value::Object(Map::from_iter(vec![
        (String::from("key1"), Value::String(String::from("value1"))),
        (String::from("key2"), Value::Bool(true)),
    ]));
    
    let type_instance = Type(&object);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let _ = type_instance.fmt(formatter);
}

#[test]
fn test_display_object_with_nested_objects() {
    use crate::map::Map;
    
    let inner_object = Value::Object(Map::from_iter(vec![
        (String::from("innerKey1"), Value::Number(Number::from(1))),
    ]));
    
    let object = Value::Object(Map::from_iter(vec![
        (String::from("outerKey"), inner_object),
    ]));
    
    let type_instance = Type(&object);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let _ = type_instance.fmt(formatter);
}

#[test]
fn test_display_empty_object() {
    use crate::map::Map;
    
    let object = Value::Object(Map::new());
    
    let type_instance = Type(&object);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let _ = type_instance.fmt(formatter);
}

