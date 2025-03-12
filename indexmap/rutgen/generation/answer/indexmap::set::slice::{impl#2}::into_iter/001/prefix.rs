// Answer 0

#[test]
fn test_into_iter_empty() {
    let slice: Box<Slice<u32>> = Box::new(Slice::new());
    let iter = (*slice).into_iter();
}

#[test]
fn test_into_iter_single_element() {
    let slice: Box<Slice<u32>> = Box::new(Slice { entries: [Bucket::new(1)] });
    let iter = (*slice).into_iter();
}

#[test]
fn test_into_iter_multiple_elements() {
    let slice: Box<Slice<u32>> = Box::new(Slice { entries: [Bucket::new(1), Bucket::new(2), Bucket::new(3)] });
    let iter = (*slice).into_iter();
}

