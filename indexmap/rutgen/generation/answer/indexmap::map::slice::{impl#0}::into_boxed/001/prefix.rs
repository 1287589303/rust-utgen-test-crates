// Answer 0

#[test]
fn test_into_boxed_non_empty() {
    let buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 1, key: 1, value: 10 },
        Bucket { hash: 2, key: 2, value: 20 },
    ];
    let slice = Box::new(Slice::from_slice(&buckets));
    let boxed_buckets: Box<[Bucket<i32, i32>]> = slice.into_boxed();
}

#[test]
fn test_into_boxed_empty() {
    let buckets: Vec<Bucket<i32, i32>> = vec![];
    let slice = Box::new(Slice::from_slice(&buckets));
    let boxed_buckets: Box<[Bucket<i32, i32>]> = slice.into_boxed();
}

