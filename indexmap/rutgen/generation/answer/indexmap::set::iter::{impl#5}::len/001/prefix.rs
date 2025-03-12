// Answer 0

#[test]
fn test_len_empty() {
    let buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = SliceIter::from(&buckets);
    let test_iter = Iter { iter };
    let _ = test_iter.len();
}

#[test]
fn test_len_single() {
    let buckets = vec![Bucket { hash: 0, key: 1, value: 10 }];
    let iter = SliceIter::from(&buckets);
    let test_iter = Iter { iter };
    let _ = test_iter.len();
}

#[test]
fn test_len_multiple() {
    let buckets = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];
    let iter = SliceIter::from(&buckets);
    let test_iter = Iter { iter };
    let _ = test_iter.len();
}

#[test]
fn test_len_max_capacity() {
    let max_capacity = 100; // Replace with the appropriate maximum capacity if known
    let buckets: Vec<Bucket<i32, i32>> = (0..max_capacity)
        .map(|i| Bucket { hash: i as u64, key: i, value: i * 10 })
        .collect();
    let iter = SliceIter::from(&buckets);
    let test_iter = Iter { iter };
    let _ = test_iter.len();
}

