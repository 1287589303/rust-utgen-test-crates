// Answer 0

#[test]
fn test_reverse_empty_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reverse();
}

#[test]
fn test_reverse_single_entry() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.reverse();
}

#[test]
fn test_reverse_multiple_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });
    map.reverse();
}

#[test]
fn test_reverse_large_number_of_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(100);
    for i in 0..100 {
        map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i * 10 });
    }
    map.reverse();
}

