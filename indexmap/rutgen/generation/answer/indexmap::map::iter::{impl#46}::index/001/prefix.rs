// Answer 0

#[test]
fn test_index_with_valid_lower_bound() {
    let buckets: Vec<Bucket<i32, &str>> = vec![
        Bucket { hash: 0, key: 1, value: "one" },
        Bucket { hash: 0, key: 2, value: "two" },
    ];
    let keys_iter = Keys { iter: buckets.iter() };
    let _ = keys_iter.index(0);
}

#[test]
fn test_index_with_valid_upper_bound() {
    let buckets: Vec<Bucket<i32, &str>> = vec![
        Bucket { hash: 0, key: 1, value: "one" },
        Bucket { hash: 0, key: 2, value: "two" },
    ];
    let keys_iter = Keys { iter: buckets.iter() };
    let _ = keys_iter.index(1);
}

#[should_panic]
#[test]
fn test_index_with_out_of_bounds_below() {
    let buckets: Vec<Bucket<i32, &str>> = vec![
        Bucket { hash: 0, key: 1, value: "one" },
        Bucket { hash: 0, key: 2, value: "two" },
    ];
    let keys_iter = Keys { iter: buckets.iter() };
    let _ = keys_iter.index(2);
}

#[should_panic]
#[test]
fn test_index_with_out_of_bounds_above() {
    let buckets: Vec<Bucket<i32, &str>> = vec![
        Bucket { hash: 0, key: 1, value: "one" },
        Bucket { hash: 0, key: 2, value: "two" },
    ];
    let keys_iter = Keys { iter: buckets.iter() };
    let _ = keys_iter.index(usize::MAX);
}

