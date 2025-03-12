// Answer 0

#[test]
fn test_from_value_valid_user() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let user: User = from_value(j).unwrap();
}

#[test]
fn test_from_value_missing_field() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
    });

    let result: Result<User, _> = from_value(j);
}

#[test]
fn test_from_value_type_mismatch() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let j = json!(42);

    let result: Result<User, _> = from_value(j);
}

#[test]
fn test_from_value_valid_bool() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct Flag {
        active: bool,
    }

    let j = json!({"active": true});

    let flag: Flag = from_value(j).unwrap();
}

#[test]
fn test_from_value_valid_number() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct NumberHolder {
        value: i32,
    }

    let j = json!({"value": 42});

    let number_holder: NumberHolder = from_value(j).unwrap();
}

#[test]
fn test_from_value_invalid_number() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct NumberHolder {
        value: u8,
    }

    let j = json!({"value": 300}); // Out of range for u8

    let result: Result<NumberHolder, _> = from_value(j);
}

#[test]
fn test_from_value_null() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct OptionalField {
        data: Option<String>,
    }

    let j = json!(null);

    let result: Result<OptionalField, _> = from_value(j);
}

#[test]
fn test_from_value_empty_object() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct EmptyStruct;

    let j = json!({});

    let empty_struct: EmptyStruct = from_value(j).unwrap();
}

#[test]
fn test_from_value_array() {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct ArrayHolder {
        items: Vec<String>,
    }

    let j = json!({"items": ["item1", "item2", "item3"]});

    let array_holder: ArrayHolder = from_value(j).unwrap();
}

