// Answer 0

#[test]
fn test_ignore_value_with_valid_null() {
    let mut deserializer = Deserializer {
        read: StrRead::new("null"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_valid_true() {
    let mut deserializer = Deserializer {
        read: StrRead::new("true"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_valid_false() {
    let mut deserializer = Deserializer {
        read: StrRead::new("false"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_valid_negative_integer() {
    let mut deserializer = Deserializer {
        read: StrRead::new("-42"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_valid_positive_integer() {
    let mut deserializer = Deserializer {
        read: StrRead::new("123"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_empty_object() {
    let mut deserializer = Deserializer {
        read: StrRead::new("{}"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_empty_array() {
    let mut deserializer = Deserializer {
        read: StrRead::new("[]"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_nested_objects() {
    let mut deserializer = Deserializer {
        read: StrRead::new("{\"key\": {\"nested\": true}}"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_whitespace_variations() {
    let mut deserializer = Deserializer {
        read: StrRead::new("  \n\t   null   \t  "),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
#[should_panic]
fn test_ignore_value_with_invalid_value() {
    let mut deserializer = Deserializer {
        read: StrRead::new("invalid"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
#[should_panic]
fn test_ignore_value_with_trailing_comma_in_array() {
    let mut deserializer = Deserializer {
        read: StrRead::new("[1, 2, 3,]"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

#[test]
#[should_panic]
fn test_ignore_value_with_trailing_comma_in_object() {
    let mut deserializer = Deserializer {
        read: StrRead::new("{\"key\": \"value\",}"),
        scratch: vec![],
        remaining_depth: 0,
    };
    deserializer.ignore_value().unwrap();
}

