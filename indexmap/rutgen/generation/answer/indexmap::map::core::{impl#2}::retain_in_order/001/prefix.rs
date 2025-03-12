// Answer 0

#[test]
fn test_retain_in_order_with_single_entry() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices.push(0); // Simulating that indices length > entries length
    map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    
    map.retain_in_order(|_key, value| {
        *value > 5
    });
}

#[test]
fn test_retain_in_order_with_multiple_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.indices.push(0); // Simulating that indices length > entries length
    map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 5 });

    map.retain_in_order(|_key, value| {
        *value > 6
    });
}

#[test]
fn test_retain_in_order_removing_all_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.indices.push(0); // Simulating that indices length > entries length
    map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 2 });
    map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 3 });

    map.retain_in_order(|_key, _value| false);
}

#[test]
fn test_retain_in_order_with_max_capacity() {
    let max_capacity = IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY;
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(max_capacity);
    for i in 0..max_capacity {
        map.indices.push(i); // Simulating that indices length > entries length
        map.entries.push(Bucket { hash: HashValue(i as u64), key: i, value: i as usize });
    }

    map.retain_in_order(|_key, value| {
        *value % 2 == 0
    });
}

