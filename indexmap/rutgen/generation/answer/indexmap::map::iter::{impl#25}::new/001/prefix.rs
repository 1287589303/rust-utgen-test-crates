// Answer 0

#[test]
fn test_into_iter_new_empty() {
    let entries: Vec<Bucket<i32, i32>> = vec![];
    let iter = IntoIter::new(entries);
}

#[test]
fn test_into_iter_new_single_entry() {
    let bucket = Bucket { hash: 0, key: 1, value: 100 };
    let entries = vec![bucket];
    let iter = IntoIter::new(entries);
}

#[test]
fn test_into_iter_new_multiple_entries() {
    let buckets: Vec<Bucket<i32, i32>> = (0..10).map(|i| Bucket { hash: i, key: i, value: i * 10 }).collect();
    let iter = IntoIter::new(buckets);
}

#[test]
fn test_into_iter_new_hundred_entries() {
    let buckets: Vec<Bucket<i32, i32>> = (0..100).map(|i| Bucket { hash: i, key: i, value: i * 10 }).collect();
    let iter = IntoIter::new(buckets);
}

#[test]
fn test_into_iter_new_with_duplicates() {
    let buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 1, value: 200 },
        Bucket { hash: 2, key: 2, value: 300 },
    ];
    let iter = IntoIter::new(buckets);
}

