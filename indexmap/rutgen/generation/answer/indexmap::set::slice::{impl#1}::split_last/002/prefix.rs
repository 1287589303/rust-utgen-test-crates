// Answer 0

#[test]
fn test_split_last_empty_slice() {
    let slice: Box<Slice<usize>> = Box::new(Slice::new());
    let result = slice.split_last();
}

#[test]
fn test_split_last_single_element() {
    let entries = [Bucket { hash: HashValue::default(), key: 1, value: 1 }];
    let slice: Box<Slice<usize>> = Box::new(Slice::from_boxed(Box::new(entries)));
    let result = slice.split_last();
}

#[test]
fn test_split_last_two_elements() {
    let entries = [
        Bucket { hash: HashValue::default(), key: 1, value: 1 },
        Bucket { hash: HashValue::default(), key: 2, value: 2 }
    ];
    let slice: Box<Slice<usize>> = Box::new(Slice::from_boxed(Box::new(entries)));
    let result = slice.split_last();
}

