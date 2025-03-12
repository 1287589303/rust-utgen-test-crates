// Answer 0

#[test]
fn test_into_iter_empty_map() {
    let map: HashMap<i32, i32> = HashMap::new();
    let iter = map.into_iter();
}

#[test]
fn test_into_iter_single_element() {
    let map: HashMap<&str, i32> = vec![("a", 1)].into_iter().collect();
    let iter = map.into_iter();
}

#[test]
fn test_into_iter_multiple_elements() {
    let map: HashMap<&str, i32> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    let iter = map.into_iter();
}

#[test]
fn test_into_iter_large_number_of_elements() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..1000 {
        map.insert(i, i * 2);
    }
    let iter = map.into_iter();
}

