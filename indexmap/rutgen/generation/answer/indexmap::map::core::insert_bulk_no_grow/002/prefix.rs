// Answer 0

#[test]
fn test_insert_bulk_no_grow_exact_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(3); // capacity is 3
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_entry_with_duplicate_hash() {
    let mut indices = hash_table::HashTable::with_capacity(5); // capacity is 5
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(1), key: 2, value: "b" }, // Duplicate hash
    ];
    insert_bulk_no_grow(&mut indices, &entries); // This may panic due to the duplicate
}

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_exceed_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(2); // capacity is 2
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" }, // Exceeding capacity
    ];
    insert_bulk_no_grow(&mut indices, &entries); // Should panic
} 

#[test]
fn test_insert_bulk_no_grow_empty_entries() {
    let mut indices = hash_table::HashTable::with_capacity(1); // capacity is 1
    let entries: Vec<Bucket<usize, &str>> = vec![]; // Empty entries
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_single_entry() {
    let mut indices = hash_table::HashTable::with_capacity(2); // capacity is 2
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

