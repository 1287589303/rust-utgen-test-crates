// Answer 0

#[test]
fn test_split_first_empty_slice() {
    let slice: Box<Slice<u32>> = Box::new(Slice { entries: [] });
    let result = slice.split_first();
}

#[test]
fn test_split_first_one_element_slice() {
    let entry = Bucket { hash: 0, key: 1, value: "value" };
    let slice: Box<Slice<&str>> = Box::new(Slice { entries: [entry] });
    let result = slice.split_first();
}

