// Answer 0

#[test]
fn last_mut_empty_slice() {
    let mut slice: Box<Slice<i32, i32>> = Box::new(Slice::new_mut());
    let result = slice.last_mut();
}

#[test]
fn last_mut_single_entry() {
    let mut slice: Box<Slice<i32, i32>> = Box::new(Slice {
        entries: [Bucket {
            hash: HashValue::default(),
            key: 1,
            value: 10,
        }],
    });
    let result = slice.last_mut();
}

#[test]
fn last_mut_multiple_entries() {
    let mut slice: Box<Slice<i32, i32>> = Box::new(Slice {
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
    });
    let result = slice.last_mut();
}

#[test]
fn last_mut_multiple_entries_modify_value() {
    let mut slice: Box<Slice<i32, i32>> = Box::new(Slice {
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
    });
    if let Some((_, value)) = slice.last_mut() {
        *value += 5;
    }
}

