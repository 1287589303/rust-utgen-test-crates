// Answer 0

#[test]
fn test_fmt_debug_single_bucket() {
    let bucket = Bucket { 
        hash: HashValue::new(1), 
        key: "key1", 
        value: "value1" 
    };
    let iter = IntoIter {
        iter: vec![bucket].into_iter(),
    };
    let _ = iter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_debug_multiple_buckets() {
    let buckets = vec![
        Bucket { hash: HashValue::new(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue::new(2), key: "key2", value: "value2" },
    ];
    let iter = IntoIter {
        iter: buckets.into_iter(),
    };
    let _ = iter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_debug_empty_iter() {
    let iter = IntoIter {
        iter: Vec::<Bucket<&str, &str>>::new().into_iter(),
    };
    let _ = iter.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_debug_hash_collisions() {
    let buckets = vec![
        Bucket { hash: HashValue::new(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue::new(1), key: "key2", value: "value2" },
    ];
    let iter = IntoIter {
        iter: buckets.into_iter(),
    };
    let _ = iter.fmt(&mut fmt::Formatter::new());
}

