// Answer 0

#[test]
fn test_append_unchecked_empty_other() {
    let mut self_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let mut other_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    self_map.append_unchecked(&mut other_map);
}

#[test]
fn test_append_unchecked_non_empty_other() {
    let mut self_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    let mut other_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    
    // Assuming entries added to other_map here somehow
    other_map.entries.push(Bucket { hash: HashValue::new(1), key: 1, value: 10 });
    
    self_map.append_unchecked(&mut other_map);
}

#[test]
fn test_append_unchecked_large_other() {
    let mut self_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(20);
    let mut other_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(15);
    
    // Fill other_map with MAX_ENTRIES_CAPACITY - 1 entries
    for i in 0..14 {
        other_map.entries.push(Bucket { hash: HashValue::new(i as u64), key: i, value: i * 10 });
    }
    
    self_map.append_unchecked(&mut other_map);
}

#[test]
#[should_panic]
fn test_append_unchecked_exceed_capacity() {
    let mut self_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    let mut other_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    
    // Fill other_map with 10 entries, which exceeds self_map capacity
    for i in 0..10 {
        other_map.entries.push(Bucket { hash: HashValue::new(i as u64), key: i, value: i * 5 });
    }
    
    self_map.append_unchecked(&mut other_map);
}

