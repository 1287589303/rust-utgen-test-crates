// Answer 0

#[test]
fn test_pointer_valid_object_navigation() {
    let data = Value::Object(Map {
        map: vec![
            (String::from("x"), Value::Object(Map {
                map: vec![
                    (String::from("y"), Value::Array(vec![Value::String(String::from("z")), Value::String(String::from("zz"))])),
                ],
            })),
        ].into_iter().collect(),
    });
    
    let result = data.pointer("/x/y/1");
}

#[test]
fn test_pointer_invalid_token() {
    let data = Value::Object(Map {
        map: vec![
            (String::from("a"), Value::Object(Map {
                map: vec![
                    (String::from("b"), Value::Null),
                ],
            })),
        ].into_iter().collect(),
    });
    
    let result = data.pointer("/a/x/y");
}

#[test]
fn test_pointer_empty_object_access() {
    let data = Value::Object(Map {
        map: vec![
            (String::from("empty"), Value::Object(Map {
                map: vec![],
            })),
        ].into_iter().collect(),
    });
    
    let result = data.pointer("/empty/nonexistent");
}

#[test]
fn test_pointer_array_access() {
    let data = Value::Array(vec![Value::Number(Number { n: 42 }), Value::String(String::from("foo"))]);
    
    let result = data.pointer("/1");
}

#[test]
fn test_pointer_with_special_characters() {
    let data = Value::Object(Map {
        map: vec![
            (String::from("special~key"), Value::String(String::from("value"))),
        ].into_iter().collect(),
    });
    
    let result = data.pointer("/special~0key");
}

