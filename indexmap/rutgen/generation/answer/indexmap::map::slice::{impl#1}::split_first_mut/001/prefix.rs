// Answer 0

#[test]
fn test_split_first_mut_single_entry() {
    let mut slice: Slice<i32, i32> = Slice {
        entries: [Bucket {
            hash: HashValue::default(),
            key: 1,
            value: 10,
        }],
    };
    slice.split_first_mut();
}

#[test]
fn test_split_first_mut_multiple_entries() {
    let mut slice: Slice<i32, i32> = Slice {
        entries: [
            Bucket {
                hash: HashValue::default(),
                key: 1,
                value: 10,
            },
            Bucket {
                hash: HashValue::default(),
                key: 2,
                value: 20,
            },
        ],
    };
    slice.split_first_mut();
}

