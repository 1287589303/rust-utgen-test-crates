// Answer 0

#[test]
fn test_to_string_pretty_with_empty_struct() {
    #[derive(serde::Serialize)]
    struct EmptyStruct;

    let value = EmptyStruct;
    let _result = serde_json::to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_simple_struct() {
    #[derive(serde::Serialize)]
    struct SimpleStruct {
        field1: String,
        field2: u32,
    }

    let value = SimpleStruct {
        field1: String::from("test"),
        field2: 42,
    };
    let _result = serde_json::to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_nested_structs() {
    #[derive(serde::Serialize)]
    struct NestedStruct {
        name: String,
        age: u32,
    }

    #[derive(serde::Serialize)]
    struct ParentStruct {
        child: NestedStruct,
    }

    let value = ParentStruct {
        child: NestedStruct {
            name: String::from("child"),
            age: 5,
        },
    };
    let _result = serde_json::to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_empty_map() {
    use serde::Serialize;
    use std::collections::HashMap;

    let value: HashMap<String, String> = HashMap::new();
    let _result = serde_json::to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_non_empty_map() {
    use serde::Serialize;
    use std::collections::HashMap;

    let mut value = HashMap::new();
    value.insert(String::from("key1"), String::from("value1"));
    value.insert(String::from("key2"), String::from("value2"));
    let _result = serde_json::to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_large_struct() {
    #[derive(serde::Serialize)]
    struct LargeStruct {
        data: Vec<u32>,
    }

    let value = LargeStruct {
        data: (0..1000).collect(),
    };
    let _result = serde_json::to_string_pretty(&value);
}

#[test]
#[should_panic]
fn test_to_string_pretty_with_invalid_serializable() {
    struct NonSerializable;

    let value = NonSerializable;
    let _result = serde_json::to_string_pretty(&value);
}

