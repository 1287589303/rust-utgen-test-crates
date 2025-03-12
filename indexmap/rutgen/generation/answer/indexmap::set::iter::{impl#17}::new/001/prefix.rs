// Answer 0

#[test]
fn test_drain_new_non_empty() {
    let mut vec_buckets: Vec<Bucket<i32>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
    ];
    let drain = vec_buckets.drain(..);
    let drain_instance = Drain::new(drain);
}

#[test]
fn test_drain_new_single_element() {
    let mut vec_buckets: Vec<Bucket<i32>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
    ];
    let drain = vec_buckets.drain(..);
    let drain_instance = Drain::new(drain);
}

#[test]
fn test_drain_new_large_vector() {
    let mut vec_buckets: Vec<Bucket<i32>> = (1..=100).map(|i| Bucket { hash: HashValue::default(), key: i, value: i * 10 }).collect();
    let drain = vec_buckets.drain(..);
    let drain_instance = Drain::new(drain);
}

#[test]
#[should_panic]
fn test_drain_new_empty() {
    let mut vec_buckets: Vec<Bucket<i32>> = Vec::new();
    let drain = vec_buckets.drain(..);
    let drain_instance = Drain::new(drain);
}

