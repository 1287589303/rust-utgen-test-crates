// Answer 0

#[test]
fn test_values_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let values = slice.values();
}

#[test]
fn test_values_single_entry() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [Bucket { hash: HashValue::default(), key: 1, value: 100 }] });
    let values = slice.values();
}

#[test]
fn test_values_multiple_entries() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 100 },
        Bucket { hash: HashValue::default(), key: 2, value: 200 },
    ]});
    let values = slice.values();
}

