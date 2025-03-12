// Answer 0

#[test]
fn test_into_values_with_non_empty_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::default(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let values_iterator = map.into_values();
}

#[test]
fn test_into_values_with_single_element_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), Global);
    map.insert("single", 42);
    
    let values_iterator = map.into_values();
}

#[test]
fn test_into_values_with_two_elements_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::default(), Global);
    map.insert("first", 10);
    map.insert("second", 20);
    
    let values_iterator = map.into_values();
}

#[test]
#[should_panic]
fn test_into_values_empty_map() {
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), Global);
    
    let values_iterator = map.into_values();
}

