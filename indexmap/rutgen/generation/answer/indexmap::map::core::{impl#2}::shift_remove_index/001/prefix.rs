// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.entries.push(Bucket { key: 0, value: "zero".to_string() });
    map.entries.push(Bucket { key: 1, value: "one".to_string() });
    map.entries.push(Bucket { key: 2, value: "two".to_string() });
    map.indices.insert(0, 0);
    map.indices.insert(1, 1);
    map.indices.insert(2, 2);
    
    let result = map.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_first() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.entries.push(Bucket { key: 0, value: "zero".to_string() });
    map.entries.push(Bucket { key: 1, value: "one".to_string() });
    
    let result = map.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_last() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.entries.push(Bucket { key: 0, value: "zero".to_string() });
    
    let result = map.shift_remove_index(0);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.entries.push(Bucket { key: 0, value: "zero".to_string() });
    
    let result = map.shift_remove_index(1);
}

