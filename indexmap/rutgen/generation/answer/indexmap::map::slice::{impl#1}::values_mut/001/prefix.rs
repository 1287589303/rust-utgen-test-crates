// Answer 0

#[test]
fn test_values_mut_non_empty_slice() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
            Bucket { hash: HashValue::default(), key: 2, value: "b" },
        ],
    });

    let _values_mut = slice.values_mut();
}

#[test]
fn test_values_mut_single_element_slice() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
        ],
    });

    let _values_mut = slice.values_mut();
}

#[test]
fn test_values_mut_multiple_elements_slice() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
            Bucket { hash: HashValue::default(), key: 2, value: "b" },
            Bucket { hash: HashValue::default(), key: 3, value: "c" },
        ],
    });

    let _values_mut = slice.values_mut();
}

