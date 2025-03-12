// Answer 0

#[test]
fn test_clone_from_increase_capacity() {
    let mut self_map: IndexMapCore<i32, i32> = IndexMapCore::new();
    let other_map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(10);

    self_map.entries.reserve(5); // set capacity to 5
    for i in 0..5 {
        self_map.entries.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 10,
        });
    }

    // other_map has a length greater than self_map's capacity
    for i in 0..10 {
        other_map.entries.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 20,
        });
    }

    self_map.clone_from(&other_map);
}

#[test]
fn test_clone_from_exact_capacity() {
    let mut self_map: IndexMapCore<i32, i32> = IndexMapCore::new();
    let other_map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(15);

    self_map.entries.reserve(8); // set capacity to 8
    for i in 0..8 {
        self_map.entries.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 30,
        });
    }

    // other_map has a length greater than self_map's capacity
    for i in 0..10 {
        other_map.entries.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 40,
        });
    }

    self_map.clone_from(&other_map);
}

#[test]
fn test_clone_from_large_map() {
    let mut self_map: IndexMapCore<i32, i32> = IndexMapCore::new();
    let other_map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(20);

    self_map.entries.reserve(12); // set capacity to 12
    for i in 0..12 {
        self_map.entries.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 50,
        });
    }

    // other_map has a length greater than self_map's capacity
    for i in 0..25 {
        other_map.entries.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 60,
        });
    }

    self_map.clone_from(&other_map);
}

