// Answer 0

#[test]
fn test_from_slice_non_empty() {
    let bucket = Bucket { hash: 0, key: 1, value: "value" };
    let entries = &[bucket];
    let slice = Slice::from_slice(entries);
}

#[test]
fn test_from_slice_empty() {
    let entries: &[Bucket<i32, &str>] = &[];
    let slice = Slice::from_slice(entries);
}

#[test]
#[should_panic]
fn test_from_slice_invalid_pointer() {
    let entries: *const Bucket<i32, &str> = std::ptr::null();
    let slice = unsafe { Slice::from_slice(&*(entries as *const [Bucket<i32, &str>])) };
}

#[test]
fn test_from_slice_large_slice() {
    let mut buckets: Vec<Bucket<i32, &str>> = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        buckets.push(Bucket { hash: i as u64, key: i, value: "value" });
    }
    let entries = buckets.as_slice();
    let slice = Slice::from_slice(entries);
}

