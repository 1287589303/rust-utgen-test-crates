// Answer 0

#[test]
fn test_len_empty_iter() {
    let buckets: Vec<Bucket<i32>> = Vec::new();
    let iter = IntoIter { iter: buckets.into_iter() };
    let _ = iter.len();
}

#[test]
fn test_len_single_element_iter() {
    let buckets = vec![Bucket { hash: HashValue::default(), key: 1, value: "one" }];
    let iter = IntoIter { iter: buckets.into_iter() };
    let _ = iter.len();
}

#[test]
fn test_len_multiple_elements_iter() {
    let buckets: Vec<Bucket<i32>> = (1..=1000)
        .map(|i| Bucket { hash: HashValue::default(), key: i, value: i.to_string() })
        .collect();
    let iter = IntoIter { iter: buckets.into_iter() };
    let _ = iter.len();
}

