// Answer 0

#[test]
fn test_insert_sorted_empty_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::default();
    let key = 10;
    let value = String::from("Ten");
    map.insert_sorted(key, value);
}

#[test]
fn test_insert_sorted_unsorted_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::default();
    map.insert(30, String::from("Thirty"));
    map.insert(20, String::from("Twenty"));
    let key = 15;
    let value = String::from("Fifteen");
    map.insert_sorted(key, value);
}

#[test]
fn test_insert_sorted_with_existing_greater_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::default();
    map.insert(5, String::from("Five"));
    map.insert(15, String::from("Fifteen"));
    map.insert(25, String::from("Twenty Five"));
    let key = 10;
    let value = String::from("Ten");
    map.insert_sorted(key, value);
}

#[test]
fn test_insert_sorted_with_existing_lower_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::default();
    map.insert(20, String::from("Twenty"));
    map.insert(30, String::from("Thirty"));
    map.insert(40, String::from("Forty"));
    let key = 10;
    let value = String::from("Ten");
    map.insert_sorted(key, value);
}

#[test]
fn test_insert_sorted_key_at_end_unsorted_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::default();
    map.insert(10, String::from("Ten"));
    map.insert(1, String::from("One"));
    let key = 20;
    let value = String::from("Twenty");
    map.insert_sorted(key, value);
}

