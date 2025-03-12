// Answer 0

#[test]
fn test_erase_indices_erased_zero() {
    let mut index_map = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(2); // Precondition: half_capacity > 0
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    
    index_map.erase_indices(0, 0);
}

#[test]
fn test_erase_indices_start_plus_shifted_equals_half_capacity() {
    let mut index_map = IndexMapCore::with_capacity(4);
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 5, value: 6 });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 7, value: 8 });

    index_map.erase_indices(1, 3);
}

#[test]
fn test_erase_indices_erased_plus_shifted_less_half_capacity() {
    let mut index_map = IndexMapCore::new();
    index_map.indices = Indices::with_capacity(4);
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 5, value: 6 });

    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());
    
    index_map.erase_indices(0, 2);
}

#[test]
fn test_erase_indices_valid_pairs_for_erased_entries() {
    let mut index_map = IndexMapCore::with_capacity(3);
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 5, value: 6 });

    index_map.erase_indices(0, 2);
}

#[test]
fn test_erase_indices_invalid_pairs_for_erased_entries() {
    let mut index_map = IndexMapCore::with_capacity(3);
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 5, value: 6 });

    index_map.erase_indices(2, 3);
}

#[test]
fn test_erase_indices_valid_pairs_for_shifted_entries() {
    let mut index_map = IndexMapCore::with_capacity(4);
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 5, value: 6 });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 7, value: 8 });
    
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());
    index_map.indices.insert_unique(4, 3, |_| unreachable!());

    index_map.erase_indices(1, 3);
}

#[test]
fn test_erase_indices_invalid_pairs_for_shifted_entries() {
    let mut index_map = IndexMapCore::new();
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });

    index_map.erase_indices(0, 1);
}

