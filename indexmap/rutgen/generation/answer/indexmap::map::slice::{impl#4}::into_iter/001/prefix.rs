// Answer 0

#[test]
fn test_into_iter_empty() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let _iter = slice.into_iter();
}

#[test]
fn test_into_iter_single() {
    let bucket = Bucket { hash: 0, key: 1, value: 100 };
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [bucket] });
    let _iter = slice.into_iter();
}

#[test]
fn test_into_iter_ten() {
    let buckets: Vec<Bucket<i32, i32>> = (0..10).map(|i| Bucket { hash: i, key: i, value: i * 10 }).collect();
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: buckets.try_into().unwrap() });
    let _iter = slice.into_iter();
}

#[test]
fn test_into_iter_max() {
    let max_size = std::usize::MAX;
    let buckets: Vec<Bucket<i32, i32>> = (0..max_size as i32).map(|i| Bucket { hash: i as u64, key: i, value: i * 10 }).collect();
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: buckets.try_into().unwrap() });
    let _iter = slice.into_iter();
}

