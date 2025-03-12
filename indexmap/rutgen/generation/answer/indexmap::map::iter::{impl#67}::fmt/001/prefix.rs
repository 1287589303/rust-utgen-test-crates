// Answer 0

#[test]
fn test_values_mut_debug_non_empty() {
    let buckets: Vec<Bucket<i32, &str>> = vec![
        Bucket { hash: HashValue::from(1), key: 1, value: "value1" },
        Bucket { hash: HashValue::from(2), key: 2, value: "value2" },
    ];
    
    let slice: &mut [Bucket<i32, &str>] = &mut buckets.as_mut_slice();
    let values_mut = ValuesMut { iter: slice.iter_mut() };
    
    let _ = fmt::Debug::fmt(&values_mut, &mut fmt::Formatter::new());
}

#[test]
fn test_values_mut_debug_empty() {
    let buckets: Vec<Bucket<i32, &str>> = Vec::new();
    
    let slice: &mut [Bucket<i32, &str>] = &mut buckets.as_mut_slice();
    let values_mut = ValuesMut { iter: slice.iter_mut() };
    
    let _ = fmt::Debug::fmt(&values_mut, &mut fmt::Formatter::new());
}

#[test]
fn test_values_mut_debug_max_capacity() {
    let mut buckets: Vec<Bucket<i32, &str>> = (0..1000).map(|i| 
        Bucket { hash: HashValue::from(i), key: i, value: &format!("value{}", i) }
    ).collect();
    
    let slice: &mut [Bucket<i32, &str>] = &mut buckets.as_mut_slice();
    let values_mut = ValuesMut { iter: slice.iter_mut() };
    
    let _ = fmt::Debug::fmt(&values_mut, &mut fmt::Formatter::new());
}

