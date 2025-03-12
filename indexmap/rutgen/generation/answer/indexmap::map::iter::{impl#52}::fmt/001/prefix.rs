// Answer 0

#[test]
fn test_fmt_with_valid_bucket() {
    struct NonPrimitiveKey {
        id: usize,
    }

    struct NonPrimitiveValue {
        name: String,
    }

    let key = NonPrimitiveKey { id: 1 };
    let value = NonPrimitiveValue { name: String::from("Value1") };
    let bucket = Bucket { hash: 0, key, value };
    
    let iter = vec![bucket];
    let into_keys = IntoKeys { iter: iter.into_iter() };

    let _ = fmt::format(format_args!("{:?}", into_keys));
}

#[test]
fn test_fmt_with_multiple_buckets() {
    struct NonPrimitiveKey {
        id: usize,
    }

    struct NonPrimitiveValue {
        name: String,
    }

    let key1 = NonPrimitiveKey { id: 1 };
    let value1 = NonPrimitiveValue { name: String::from("Value1") };
    let bucket1 = Bucket { hash: 0, key: key1, value: value1 };

    let key2 = NonPrimitiveKey { id: 2 };
    let value2 = NonPrimitiveValue { name: String::from("Value2") };
    let bucket2 = Bucket { hash: 1, key: key2, value: value2 };

    let iter = vec![bucket1, bucket2];
    let into_keys = IntoKeys { iter: iter.into_iter() };

    let _ = fmt::format(format_args!("{:?}", into_keys));
}

#[test]
fn test_fmt_with_one_element_key() {
    struct NonPrimitiveKey {
        id: usize,
    }

    struct NonPrimitiveValue {
        name: String,
    }

    let key = NonPrimitiveKey { id: 1 };
    let value = NonPrimitiveValue { name: String::from("SingleValue") };
    let bucket = Bucket { hash: 0, key, value };

    let iter = vec![bucket];
    let into_keys = IntoKeys { iter: iter.into_iter() };

    let _ = fmt::format(format_args!("{:?}", into_keys));
}

