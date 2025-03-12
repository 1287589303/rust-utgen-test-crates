// Answer 0

#[test]
fn test_from_reader_valid_json() {
    use std::io::Cursor;
    use serde::Deserialize;
    
    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }
    
    let data = r#"{"fingerprint": "abc123", "location": "Earth"}"#;
    let reader = Cursor::new(data);
    let user: User = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_empty_input() {
    use std::io::Cursor;
    use serde::Deserialize;
    
    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }
    
    let data = r#""#;
    let reader = Cursor::new(data);
    let result: Result<User, _> = serde_json::from_reader(reader);
}

#[test]
fn test_from_reader_malformed_json() {
    use std::io::Cursor;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let data = r#"{"fingerprint": "abc123", "location": "Earth""#; // Missing closing brace
    let reader = Cursor::new(data);
    let result: Result<User, _> = serde_json::from_reader(reader);
}

#[test]
fn test_from_reader_invalid_type() {
    use std::io::Cursor;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let data = r#"[{"fingerprint": "abc123", "location": "Earth"}]"#; // Expected object, got array
    let reader = Cursor::new(data);
    let result: Result<User, _> = serde_json::from_reader(reader);
}

#[test]
fn test_from_reader_large_input() {
    use std::io::Cursor;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct LargeUser {
        number: Vec<u8>,
    }

    let data = r#"{"number": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]}"#;
    let reader = Cursor::new(data);
    let large_user: LargeUser = serde_json::from_reader(reader).unwrap();
}

