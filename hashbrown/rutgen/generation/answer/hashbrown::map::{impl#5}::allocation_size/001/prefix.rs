// Answer 0

#[test]
fn test_allocation_size_empty() {
    let map: HashMap<i32, String> = HashMap::default();
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_one_entry() {
    let mut map: HashMap<i32, String> = HashMap::default();
    map.insert(1, String::from("one"));
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_ten_entries() {
    let mut map: HashMap<i32, String> = HashMap::default();
    for i in 0..10 {
        map.insert(i, format!("value {}", i));
    }
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_one_thousand_entries() {
    let mut map: HashMap<i32, String> = HashMap::default();
    for i in 0..1000 {
        map.insert(i, format!("value {}", i));
    }
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_after_removal() {
    let mut map: HashMap<i32, String> = HashMap::default();
    for i in 0..100 {
        map.insert(i, format!("value {}", i));
    }
    map.remove(&50);
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_after_clear() {
    let mut map: HashMap<i32, String> = HashMap::default();
    for i in 0..100 {
        map.insert(i, format!("value {}", i));
    }
    map.clear();
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_with_string_keys() {
    let mut map: HashMap<String, i32> = HashMap::default();
    map.insert(String::from("key1"), 1);
    map.insert(String::from("key2"), 2);
    let size = map.allocation_size();
}

#[test]
fn test_allocation_size_after_resize() {
    let mut map: HashMap<i32, String> = HashMap::default();
    for i in 0..16 {
        map.insert(i, format!("value {}", i));
    }
    let size = map.allocation_size();
    for i in 16..32 {
        map.insert(i, format!("value {}", i));
    }
    let new_size = map.allocation_size();
}

