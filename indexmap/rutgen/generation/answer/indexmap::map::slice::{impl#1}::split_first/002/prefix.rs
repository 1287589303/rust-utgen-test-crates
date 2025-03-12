// Answer 0

#[test]
fn test_split_first_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let result = slice.split_first();
}

#[test]
fn test_split_first_single_element() {
    let entry = Bucket { hash: HashValue::default(), key: 1, value: 2 };
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [entry] });
    let result = slice.split_first();
}

