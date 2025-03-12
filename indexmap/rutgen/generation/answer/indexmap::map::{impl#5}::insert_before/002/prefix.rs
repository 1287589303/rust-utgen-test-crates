// Answer 0

#[test]
fn test_insert_before_with_occupied_entry() {
    let mut map: IndexMap<char, i32> = IndexMap::with_capacity_and_hasher(10, RandomState::new());

    // Initial entries
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    // Precondition setup: Ensure index == len
    let len = map.len();
    let key = 'b'; // this key is already occupied
    let value = 20;

    // Precondition: self.entry(key) matches Entry::Occupied
    let pre_insert_index = map.get_index_of(&key).unwrap(); // Index of 'b'

    // Precondition: index > entry.index()
    let index = pre_insert_index + 1; // This should be greater than the index of 'b'

    // Call the function under test
    let result = map.insert_before(index, key, value);

    // The expected index should equal pre_insert_index because 'b' moved
    assert_eq!(result, (pre_insert_index, Some(2))); // Old value of 'b' was 2
}

#[test]
fn test_insert_before_with_occupied_entry_at_end() {
    let mut map: IndexMap<char, i32> = IndexMap::with_capacity_and_hasher(10, RandomState::new());

    // Initial entries
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    // Precondition setup: Ensure index == len
    let len = map.len();
    let key = 'c'; // this key is already occupied
    let value = 30;

    // Precondition: self.entry(key) matches Entry::Occupied
    let pre_insert_index = map.get_index_of(&key).unwrap(); // Index of 'c'

    // Precondition: index > entry.index()
    let index = pre_insert_index + 1; // This should be greater than the index of 'c'

    // Call the function under test
    let result = map.insert_before(index, key, value);

    // The expected index should equal pre_insert_index because 'c' moved
    assert_eq!(result, (pre_insert_index, Some(3))); // Old value of 'c' was 3
}

