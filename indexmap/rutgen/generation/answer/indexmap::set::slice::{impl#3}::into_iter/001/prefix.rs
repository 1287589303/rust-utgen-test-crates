// Answer 0

#[test]
fn test_into_iter_non_empty_slice() {
    let buckets = vec![
        Bucket { hash: HashValue::new(1), key: 1, value: "a" },
        Bucket { hash: HashValue::new(2), key: 2, value: "b" },
    ];
    let slice = Box::new(Slice { entries: buckets });
    let iter = slice.into_iter();
}

#[test]
fn test_into_iter_single_element_slice() {
    let buckets = vec![
        Bucket { hash: HashValue::new(1), key: 1, value: "a" },
    ];
    let slice = Box::new(Slice { entries: buckets });
    let iter = slice.into_iter();
}

#[test]
fn test_into_iter_multiple_element_slice() {
    let buckets = vec![
        Bucket { hash: HashValue::new(3), key: 3, value: "c" },
        Bucket { hash: HashValue::new(4), key: 4, value: "d" },
        Bucket { hash: HashValue::new(5), key: 5, value: "e" },
    ];
    let slice = Box::new(Slice { entries: buckets });
    let iter = slice.into_iter();
}

