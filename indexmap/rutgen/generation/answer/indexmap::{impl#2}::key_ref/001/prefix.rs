// Answer 0

#[test]
fn test_key_ref_integer() {
    let bucket = Bucket {
        hash: HashValue(1),
        key: 42,
        value: "value",
    };
    let key_ref = bucket.key_ref();
}

#[test]
fn test_key_ref_char() {
    let bucket = Bucket {
        hash: HashValue(2),
        key: 'a',
        value: "value",
    };
    let key_ref = bucket.key_ref();
}

#[test]
fn test_key_ref_tuple() {
    let bucket = Bucket {
        hash: HashValue(3),
        key: (1, 2),
        value: "value",
    };
    let key_ref = bucket.key_ref();
}

