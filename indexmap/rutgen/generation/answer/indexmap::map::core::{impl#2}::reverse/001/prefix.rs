// Answer 0

#[test]
fn test_reverse_with_valid_indices() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(5);
    map.entries.push(Bucket { hash: 0.into(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: 1.into(), key: 2, value: 20 });
    map.entries.push(Bucket { hash: 2.into(), key: 3, value: 30 });
    map.indices.push(0);
    map.indices.push(1);
    map.indices.push(2);
    
    map.reverse();
}

#[test]
fn test_reverse_with_empty_entries() {
    let mut map = IndexMapCore::<usize, usize>::new();
    
    map.reverse();
}

#[test]
fn test_reverse_with_only_one_entry() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(1);
    map.entries.push(Bucket { hash: 0.into(), key: 1, value: 10 });
    map.indices.push(0);
    
    map.reverse();
}

