// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let mut buckets: [Bucket<&str, i32>; 3] = [
        Bucket { hash: 1, key: "key1", value: 100 },
        Bucket { hash: 2, key: "key2", value: 200 },
        Bucket { hash: 3, key: "key3", value: 300 },
    ];
    let iter_mut = IterMut2::new(&mut buckets);
    let slice = iter_mut.as_slice();
}

#[test]
fn test_as_slice_single_element() {
    let mut buckets: [Bucket<&str, i32>; 1] = [
        Bucket { hash: 1, key: "key1", value: 100 },
    ];
    let iter_mut = IterMut2::new(&mut buckets);
    let slice = iter_mut.as_slice();
}

#[test]
fn test_as_slice_empty() {
    let mut buckets: [Bucket<&str, i32>; 0] = [];
    let iter_mut = IterMut2::new(&mut buckets);
    let slice = iter_mut.as_slice();
}

#[test]
fn test_as_slice_large_slice() {
    let mut buckets: [Bucket<&str, i32>; 10] = [
        Bucket { hash: 1, key: "key1", value: 100 },
        Bucket { hash: 2, key: "key2", value: 200 },
        Bucket { hash: 3, key: "key3", value: 300 },
        Bucket { hash: 4, key: "key4", value: 400 },
        Bucket { hash: 5, key: "key5", value: 500 },
        Bucket { hash: 6, key: "key6", value: 600 },
        Bucket { hash: 7, key: "key7", value: 700 },
        Bucket { hash: 8, key: "key8", value: 800 },
        Bucket { hash: 9, key: "key9", value: 900 },
        Bucket { hash: 10, key: "key10", value: 1000 },
    ];
    let iter_mut = IterMut2::new(&mut buckets);
    let slice = iter_mut.as_slice();
}

