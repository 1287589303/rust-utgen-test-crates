// Answer 0

#[test]
fn test_iter_mut_new_single_entry() {
    let mut entries = [Bucket { hash: 0, key: 1, value: "value1" }];
    let iter_mut = IterMut::new(&mut entries);
}

#[test]
fn test_iter_mut_new_multiple_entries() {
    let mut entries = [
        Bucket { hash: 0, key: 1, value: "value1" },
        Bucket { hash: 1, key: 2, value: "value2" },
        Bucket { hash: 2, key: 3, value: "value3" },
    ];
    let iter_mut = IterMut::new(&mut entries);
}

#[test]
#[should_panic]
fn test_iter_mut_new_empty_slice() {
    let mut entries: &mut [Bucket<i32, &str>] = &mut [];
    let iter_mut = IterMut::new(entries);
}

#[test]
fn test_iter_mut_new_maximum_capacity() {
    let mut entries: Vec<Bucket<i32, &str>> = (0..usize::MAX / std::mem::size_of::<Bucket<i32, &str>>())
        .map(|i| Bucket { hash: i as u64, key: i, value: "value" })
        .collect();
    let iter_mut = IterMut::new(&mut entries);
}

