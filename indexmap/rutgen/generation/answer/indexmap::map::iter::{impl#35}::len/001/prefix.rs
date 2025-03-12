// Answer 0

#[test]
fn test_len_empty() {
    let bucket: Vec<Bucket<i32, i32>> = Vec::new();
    let mut drain_iter = bucket.drain(..);
    let drain = Drain { iter: drain_iter };
    let _ = drain.len();
}

#[test]
fn test_len_single_element() {
    let mut bucket: Vec<Bucket<i32, i32>> = vec![Bucket { hash: 0, key: 1, value: 100 }];
    let mut drain_iter = bucket.drain(..);
    let drain = Drain { iter: drain_iter };
    let _ = drain.len();
}

#[test]
fn test_len_multiple_elements() {
    let mut bucket: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 2, value: 200 },
        Bucket { hash: 2, key: 3, value: 300 },
    ];
    let mut drain_iter = bucket.drain(..);
    let drain = Drain { iter: drain_iter };
    let _ = drain.len();
}

#[test]
fn test_len_full_capacity() {
    let mut bucket: Vec<Bucket<i32, i32>> = (0..10).map(|i| Bucket { hash: i, key: i, value: i * 10 }).collect();
    let mut drain_iter = bucket.drain(..);
    let drain = Drain { iter: drain_iter };
    let _ = drain.len();
}

#[test]
fn test_len_after_draining() {
    let mut bucket: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 2, value: 200 },
    ];
    let mut drain_iter = bucket.drain(..);
    let _ = drain.len();
    let _ = drain_iter.next(); // Drain one element
    let drain = Drain { iter: drain_iter };
    let _ = drain.len();
}

