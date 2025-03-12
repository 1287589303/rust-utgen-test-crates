// Answer 0

#[test]
fn test_pointer_mut_with_single_token() {
    let mut value = Value::Object(Map::new());
    value.as_object_mut().unwrap().insert("foo".to_string(), Value::Number(Number { n: 1 }));
    value.pointer_mut("/foo");
}

#[test]
fn test_pointer_mut_with_multiple_tokens() {
    let mut value = Value::Object(Map::new());
    value.as_object_mut().unwrap().insert("a".to_string(), Value::Object(Map::new()));
    if let Value::Object(ref mut obj) = value.pointer_mut("/a").unwrap() {
        obj.insert("b".to_string(), Value::Number(Number { n: 2 }));
    }
    value.pointer_mut("/a/b");
}

#[test]
fn test_pointer_mut_with_array_index() {
    let mut value = Value::Array(Vec::new());
    value.as_array_mut().unwrap().push(Value::Number(Number { n: 3 }));
    value.pointer_mut("/0");
}

#[test]
fn test_pointer_mut_with_escaped_characters() {
    let mut value = Value::Object(Map::new());
    value.as_object_mut().unwrap().insert("foo/bar".to_string(), Value::Number(Number { n: 4 }));
    value.pointer_mut("/foo~1bar");
}

#[test]
fn test_pointer_mut_with_nested_object() {
    let mut value = Value::Object(Map::new());
    {
        let obj = value.as_object_mut().unwrap();
        obj.insert("outer".to_string(), Value::Object(Map::new()));
        if let Value::Object(ref mut inner) = obj.get_mut("outer").unwrap() {
            inner.insert("inner".to_string(), Value::Number(Number { n: 5 }));
        }
    }
    value.pointer_mut("/outer/inner");
}

