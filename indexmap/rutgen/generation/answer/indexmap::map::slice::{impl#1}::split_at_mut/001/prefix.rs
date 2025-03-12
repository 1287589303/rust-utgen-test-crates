// Answer 0

#[test]
fn test_split_at_mut_zero_index() {
    let mut slice = Box::new(Slice {
        entries: [Bucket { hash: HashValue::default(), key: "key1", value: "value1" }],
    });
    slice.split_at_mut(0);
}

#[test]
fn test_split_at_mut_full_length() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: "key1", value: "value1" },
            Bucket { hash: HashValue::default(), key: "key2", value: "value2" },
        ],
    });
    slice.split_at_mut(2);
}

#[test]
#[should_panic]
fn test_split_at_mut_out_of_bounds() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: "key1", value: "value1" },
            Bucket { hash: HashValue::default(), key: "key2", value: "value2" },
        ],
    });
    slice.split_at_mut(3);
}

