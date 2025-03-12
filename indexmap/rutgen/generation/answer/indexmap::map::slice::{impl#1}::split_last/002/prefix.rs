// Answer 0

#[test]
fn test_split_last_empty_entries() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let result = slice.split_last();
}

#[test]
fn test_split_last_single_element() {
    let bucket = Bucket { hash: 0, key: 1, value: 2 };
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [bucket] });
    let result = slice.split_last();
}

