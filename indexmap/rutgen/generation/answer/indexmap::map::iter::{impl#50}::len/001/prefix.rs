// Answer 0

#[test]
fn test_len_empty_container() {
    let buckets: Vec<Bucket<i32, i32>> = vec![];
    let iter = IntoKeys {
        iter: buckets.into_iter()
    };
    let _ = iter.len();
}

#[test]
fn test_len_single_bucket() {
    let buckets = vec![Bucket { hash: 0, key: 1, value: 100 }];
    let iter = IntoKeys {
        iter: buckets.into_iter()
    };
    let _ = iter.len();
}

#[test]
fn test_len_multiple_buckets() {
    let buckets = vec![
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 2, value: 200 },
        Bucket { hash: 2, key: 3, value: 300 },
        Bucket { hash: 3, key: 4, value: 400 },
        Bucket { hash: 4, key: 5, value: 500 },
    ];
    let iter = IntoKeys {
        iter: buckets.into_iter()
    };
    let _ = iter.len();
}

