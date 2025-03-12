// Answer 0

#[test]
fn test_fmt_non_empty_vector() {
    let bucket1 = Bucket { hash: HashValue::from(1), key: "key1", value: 10 };
    let bucket2 = Bucket { hash: HashValue::from(2), key: "key2", value: 20 };
    
    let vec = vec![bucket1, bucket2];
    let mut drain = vec.clone().drain(..); // Create a drain iterator from the vector

    let drain_instance = Drain { iter: drain };
    let mut formatter = fmt::Formatter::new();

    drain_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_single_bucket() {
    let bucket = Bucket { hash: HashValue::from(3), key: "key3", value: 30 };

    let vec = vec![bucket];
    let mut drain = vec.clone().drain(..);

    let drain_instance = Drain { iter: drain };
    let mut formatter = fmt::Formatter::new();

    drain_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_multiple_buckets() {
    let bucket1 = Bucket { hash: HashValue::from(4), key: "key4", value: 40 };
    let bucket2 = Bucket { hash: HashValue::from(5), key: "key5", value: 50 };
    let bucket3 = Bucket { hash: HashValue::from(6), key: "key6", value: 60 };

    let vec = vec![bucket1, bucket2, bucket3];
    let mut drain = vec.clone().drain(..);

    let drain_instance = Drain { iter: drain };
    let mut formatter = fmt::Formatter::new();

    drain_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_different_types() {
    let bucket = Bucket { hash: HashValue::from(7), key: "key7", value: "value7" }; // Here value is a string

    let vec = vec![bucket];
    let mut drain = vec.clone().drain(..);

    let drain_instance = Drain { iter: drain };
    let mut formatter = fmt::Formatter::new();

    drain_instance.fmt(&mut formatter);
}

