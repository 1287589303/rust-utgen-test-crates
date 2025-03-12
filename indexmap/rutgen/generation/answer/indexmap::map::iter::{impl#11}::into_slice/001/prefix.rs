// Answer 0

#[test]
fn test_into_slice_empty() {
    let mut empty_entries: [Bucket<i32, i32>; 0] = [];
    let iter = IterMut::new(&mut empty_entries);
    let slice = iter.into_slice();
}

#[test]
fn test_into_slice_single_entry() {
    let mut single_entries = [Bucket { hash: 0, key: 1, value: 100 }];
    let iter = IterMut::new(&mut single_entries);
    let slice = iter.into_slice();
}

#[test]
fn test_into_slice_multiple_entries() {
    let mut multiple_entries = [
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 2, value: 200 },
        Bucket { hash: 2, key: 3, value: 300 },
    ];
    let iter = IterMut::new(&mut multiple_entries);
    let slice = iter.into_slice();
}

#[test]
fn test_into_slice_boundary_max_size() {
    let mut max_entries: Vec<Bucket<i32, i32>> = (1..=100).map(|i| Bucket { hash: i as u64, key: i, value: i * 10 }).collect();
    let iter = IterMut::new(max_entries.as_mut_slice());
    let slice = iter.into_slice();
}

