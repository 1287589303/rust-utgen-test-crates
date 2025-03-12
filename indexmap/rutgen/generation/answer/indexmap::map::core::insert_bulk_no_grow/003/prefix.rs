// Answer 0

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_capacity_equals_length() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(2);
    let entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: "A" },
        Bucket { hash: HashValue(1), key: 2, value: "B" },
    ];
    // Set capacity to be equal to the length of entries
    indices.reserve(2);
    // Now insert will panic since capacity == len + entries.len()
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_capacity_less_than_length() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(1);
    let entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: "A" },
        Bucket { hash: HashValue(1), key: 2, value: "B" },
        Bucket { hash: HashValue(2), key: 3, value: "C" },
    ];
    // Capacity is less than the total number of entries
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_negative_space() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(0);
    let entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: "A" },
    ];
    // Indices starts from 0 capacity and 0 length, panic expected
    insert_bulk_no_grow(&mut indices, &entries);
}

