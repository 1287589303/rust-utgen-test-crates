// Answer 0

#[test]
fn test_hash_map_new_empty() {
    let map: HashMap<&str, i32> = HashMap::new();
}

#[test]
fn test_hash_map_new_with_zero_capacity() {
    let map: HashMap<&str, i32> = HashMap::with_capacity(0);
}

#[test]
fn test_hash_map_new_with_non_zero_capacity() {
    let map: HashMap<&str, i32> = HashMap::with_capacity(5);
}

