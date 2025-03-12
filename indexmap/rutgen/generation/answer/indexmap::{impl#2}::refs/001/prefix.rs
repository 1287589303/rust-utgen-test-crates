// Answer 0

#[test]
fn test_refs_with_integer_key_value() {
    let bucket = Bucket {
        hash: HashValue(1),
        key: 42,
        value: "value",
    };
    let (key_ref, value_ref) = bucket.refs();
}

#[test]
fn test_refs_with_float_key_value() {
    let bucket = Bucket {
        hash: HashValue(2),
        key: 3.14,
        value: "float_value",
    };
    let (key_ref, value_ref) = bucket.refs();
}

#[test]
fn test_refs_with_char_key_value() {
    let bucket = Bucket {
        hash: HashValue(3),
        key: 'a',
        value: "char_value",
    };
    let (key_ref, value_ref) = bucket.refs();
}

#[test]
fn test_refs_with_string_key_value() {
    let bucket = Bucket {
        hash: HashValue(4),
        key: String::from("key_string"),
        value: 100,
    };
    let (key_ref, value_ref) = bucket.refs();
}

#[test]
fn test_refs_with_struct_key_value() {
    #[derive(Copy, Clone)]
    struct KeyStruct {
        id: u32,
    }

    let bucket = Bucket {
        hash: HashValue(5),
        key: KeyStruct { id: 1 },
        value: "struct_value",
    };
    let (key_ref, value_ref) = bucket.refs();
}

