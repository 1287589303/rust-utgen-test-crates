// Answer 0

#[test]
fn test_entry_occupied_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    let key = 'a';
    let entry = map.entry(key);
}

#[test]
fn test_entry_occupied_another_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("apple", 1);
    map.insert("banana", 2);
    map.insert("cherry", 3);

    let key = "banana";
    let entry = map.entry(key);
}

#[test]
fn test_entry_occupied_with_multiple_keys() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);

    let key = 2;
    let entry = map.entry(key);
}

