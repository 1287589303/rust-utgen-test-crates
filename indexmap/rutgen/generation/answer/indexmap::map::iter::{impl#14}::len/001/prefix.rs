// Answer 0

#[test]
fn test_iter_mut_len_zero() {
    let empty_buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = IterMut {
        iter: empty_buckets.iter_mut(),
    };
    let length = iter.len();
}

#[test]
fn test_iter_mut_len_one() {
    let buckets = vec![Bucket { hash: HashValue::default(), key: 1, value: 1 }];
    let iter = IterMut {
        iter: buckets.iter_mut(),
    };
    let length = iter.len();
}

#[test]
fn test_iter_mut_len_boundary() {
    let buckets: Vec<Bucket<i32, i32>> = (1..=1000).map(|i| Bucket { hash: HashValue::default(), key: i, value: i }).collect();
    let iter = IterMut {
        iter: buckets.iter_mut(),
    };
    let length = iter.len();
}

