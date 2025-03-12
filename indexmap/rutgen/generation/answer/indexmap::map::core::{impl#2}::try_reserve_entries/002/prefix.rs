// Answer 0

#[test]
fn test_try_reserve_entries_additional_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    let additional = 0;
    let _ = map.try_reserve_entries(additional);
}

#[test]
fn test_try_reserve_entries_additional_one() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    let additional = 1;
    let _ = map.try_reserve_entries(additional);
}

#[test]
fn test_try_reserve_entries_additional_exceeding_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });

    let additional = 2; // triggers capacity constraints
    let _ = map.try_reserve_entries(additional);
}

