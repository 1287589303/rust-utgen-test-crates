// Answer 0

#[test]
fn test_borrow_mut_empty() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ref_mut = map.borrow_mut();
}

#[test]
fn test_borrow_mut_with_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    let _ref_mut = map.borrow_mut();
}

#[test]
fn test_borrow_mut_after_insertion() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    let _ref_mut = map.borrow_mut();
}

#[test]
fn test_borrow_mut_after_clearing() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.clear();
    let _ref_mut = map.borrow_mut();
}

