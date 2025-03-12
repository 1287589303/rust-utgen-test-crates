// Answer 0

#[test]
fn test_erase_indices_case_erased_zero() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices.reserve(2); // Ensure there's capacity
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 20 });

    index_map.erase_indices(0, 0);
}

#[test]
fn test_erase_indices_case_start_plus_shifted_equals_half_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    index_map.indices.insert_unique(0, 0, |_| unreachable!());
    index_map.indices.insert_unique(1, 1, |_| unreachable!());
    
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 20 });

    index_map.erase_indices(2, 2);
}

#[test]
fn test_erase_indices_case_erased_plus_shifted_equals_half_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(4);
    index_map.indices.insert_unique(0, 0, |_| unreachable!());
    index_map.indices.insert_unique(1, 1, |_| unreachable!());
    index_map.indices.insert_unique(2, 2, |_| unreachable!());
    
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 20 });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: 30 });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 4, value: 40 });

    index_map.erase_indices(2, 4);
}

