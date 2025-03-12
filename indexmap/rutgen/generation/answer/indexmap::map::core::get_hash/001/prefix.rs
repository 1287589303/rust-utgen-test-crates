// Answer 0

#[test]
fn test_get_hash_valid_index() {
    let entries = vec![
        Bucket { hash: HashValue::new(10), key: 1, value: 'a' },
        Bucket { hash: HashValue::new(20), key: 2, value: 'b' },
        Bucket { hash: HashValue::new(30), key: 3, value: 'c' },
    ];
    let hash_fn = get_hash(&entries);
    let _ = hash_fn(&0);
    let _ = hash_fn(&1);
    let _ = hash_fn(&2);
}

#[test]
#[should_panic]
fn test_get_hash_out_of_bounds_index_too_high() {
    let entries = vec![
        Bucket { hash: HashValue::new(10), key: 1, value: 'a' },
        Bucket { hash: HashValue::new(20), key: 2, value: 'b' },
    ];
    let hash_fn = get_hash(&entries);
    let _ = hash_fn(&2);
}

#[test]
#[should_panic]
fn test_get_hash_out_of_bounds_index_negative() {
    let entries = vec![
        Bucket { hash: HashValue::new(10), key: 1, value: 'a' },
    ];
    let hash_fn = get_hash(&entries);
    let _ = hash_fn(&-1);
}

