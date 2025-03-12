// Answer 0

#[test]
fn test_reserve_with_additional_greater_than_capacity() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(2);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    // Initial capacity is 2, len is 2, so additional must be > 0
    let additional = 1; 
    map.reserve(additional);
}

#[test]
fn test_reserve_with_additional_greater_than_capacity_boundary() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(3);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });
    
    // Initial capacity is 3, len is 3, additional must be 1
    let additional = 1; 
    map.reserve(additional);
}

#[test]
fn test_reserve_with_additional_under_max_capacity() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(5);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    // Initial capacity is 5, len is 2, additional can be 3
    let additional = 3; 
    map.reserve(additional);
}

