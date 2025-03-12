// Answer 0

#[test]
fn test_as_slice_empty() {
    let entries: Vec<Bucket<u32, u32>> = vec![];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
}

#[test]
fn test_as_slice_one_element() {
    let entries = vec![Bucket { hash: 1, key: 1u32, value: 10u32 }];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
}

#[test]
fn test_as_slice_multiple_elements() {
    let entries = vec![
        Bucket { hash: 1, key: 1u32, value: 10u32 },
        Bucket { hash: 2, key: 2u32, value: 20u32 },
        Bucket { hash: 3, key: 3u32, value: 30u32 },
    ];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
}

#[test]
fn test_as_slice_large_vector() {
    let mut entries = Vec::with_capacity(1000);
    for i in 0..1000 {
        entries.push(Bucket { hash: i as u64, key: i, value: i * 10 });
    }
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
}

