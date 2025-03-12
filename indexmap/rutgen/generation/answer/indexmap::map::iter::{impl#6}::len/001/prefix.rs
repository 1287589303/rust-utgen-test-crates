// Answer 0

#[test]
fn test_iter_len_empty() {
    let buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

#[test]
fn test_iter_len_single() {
    let buckets = vec![Bucket { hash: 0, key: 1, value: 10 }];
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

#[test]
fn test_iter_len_two() {
    let buckets = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

#[test]
fn test_iter_len_five() {
    let buckets = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
        Bucket { hash: 3, key: 4, value: 40 },
        Bucket { hash: 4, key: 5, value: 50 },
    ];
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

#[test]
fn test_iter_len_ten() {
    let buckets = (0..10)
        .map(|i| Bucket { hash: i, key: i, value: i * 10 })
        .collect::<Vec<_>>();
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

// Assuming maximum capacity for this example is 100
#[test]
fn test_iter_len_max_capacity() {
    let buckets = (0..100)
        .map(|i| Bucket { hash: i, key: i, value: i * 10 })
        .collect::<Vec<_>>();
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

// Extreme type cases, using String as K and V
#[test]
fn test_iter_len_extreme_types() {
    let buckets = vec![
        Bucket {
            hash: 0,
            key: String::from("key1"),
            value: String::from("value1"),
        },
        Bucket {
            hash: 1,
            key: String::from("key2"),
            value: String::from("value2"),
        },
    ];
    let iter = Iter { iter: buckets.iter() };
    let length = iter.len();
}

