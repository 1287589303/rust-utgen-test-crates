// Answer 0

#[test]
fn test_move_index_from_less_than_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 1);
}

#[test]
fn test_move_index_from_greater_than_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(2, 1);
}

#[test]
fn test_move_index_from_equals_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 1);
}

