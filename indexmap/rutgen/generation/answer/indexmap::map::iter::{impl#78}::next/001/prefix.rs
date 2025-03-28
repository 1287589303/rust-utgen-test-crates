// Answer 0

#[test]
fn test_next_empty_drain() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = 
        IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: std::collections::hash_map::RandomState::new() };
    let iter = vec![].into_iter(); // Empty iterator
    let mut splice = Splice { map: &mut map, tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, drain: vec::IntoIter::from(vec![]), replace_with: iter };
    
    let result = splice.next();
    assert_eq!(result, None);
}

#[test]
fn test_next_single_bucket() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = 
        IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: std::collections::hash_map::RandomState::new() };
    
    let bucket = Bucket { hash: HashValue::new(1), key: 1, value: 10 };
    let iter = vec![bucket].into_iter(); // Single bucket
    let mut splice = Splice { map: &mut map, tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, drain: vec::IntoIter::from(vec![bucket]), replace_with: iter };
    
    let result = splice.next();
    assert_eq!(result, Some((1, 10)));
}

#[test]
fn test_next_multiple_buckets() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = 
        IndexMap { core: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, hash_builder: std::collections::hash_map::RandomState::new() };
    
    let bucket1 = Bucket { hash: HashValue::new(1), key: 1, value: 10 };
    let bucket2 = Bucket { hash: HashValue::new(2), key: 2, value: 20 };
    let iter = vec![(3, 30)].into_iter(); // Replace with other bucket
    let mut splice = Splice { map: &mut map, tail: IndexMapCore { indices: Indices::new(), entries: Entries::new() }, drain: vec::IntoIter::from(vec![bucket1, bucket2]), replace_with: iter };
    
    let result1 = splice.next();
    assert_eq!(result1, Some((1, 10)));
    
    let result2 = splice.next();
    assert_eq!(result2, Some((2, 20)));
    
    let result3 = splice.next();
    assert_eq!(result3, None);
}

