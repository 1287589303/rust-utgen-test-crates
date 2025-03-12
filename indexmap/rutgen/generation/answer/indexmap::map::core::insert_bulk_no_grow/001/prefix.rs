// Answer 0

#[test]
fn test_insert_bulk_no_grow_with_sufficient_capacity_true() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(5);
    // Simulate capacity 5, and length 0 for total of 5 - 0 >= 5 entries
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
        Bucket { hash: HashValue(4), key: 4, value: "d" },
        Bucket { hash: HashValue(5), key: 5, value: "e" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_with_sufficient_capacity_false() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(5);
    // Simulate capacity 5, and length 0 for total of 5 - 0 >= 1 entry
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[should_panic]
#[test]
fn test_insert_bulk_no_grow_with_insufficient_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(5);
    // Too many entries for the current capacity of the indices
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
        Bucket { hash: HashValue(4), key: 4, value: "d" },
        Bucket { hash: HashValue(5), key: 5, value: "e" },
        Bucket { hash: HashValue(6), key: 6, value: "f" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

