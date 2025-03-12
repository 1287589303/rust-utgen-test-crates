// Answer 0

#[test]
fn test_get_full_existing_key_first_entry() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    let result = map.get_full(&1);
}

#[test]
fn test_get_full_existing_key_middle_entry() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    let result = map.get_full(&3);
}

#[test]
fn test_get_full_existing_key_last_entry() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(5, "five");
    let result = map.get_full(&5);
}

#[test]
fn test_get_full_existing_key_duplicate_index() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(6, "six");
    map.insert(7, "seven");
    let result = map.get_full(&6);
}

