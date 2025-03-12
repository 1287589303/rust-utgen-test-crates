// Answer 0

#[test]
fn test_extend_empty_iterator() {
    let mut map = HashMap::new();
    let empty_iter: Vec<(i32, i32)> = Vec::new();
    map.extend(empty_iter.iter());
}

#[test]
fn test_extend_single_element_iterator() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let single_elem = vec![(1, 200)];
    map.extend(single_elem.iter());
    let result = map.get(&1);
}

#[test]
fn test_extend_multiple_elements_iterator() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    let multiple_elems = vec![(1, 150), (3, 300), (4, 400)];
    map.extend(multiple_elems.iter());
    let result_1 = map.get(&1);
    let result_3 = map.get(&3);
    let result_4 = map.get(&4);
}

#[test]
fn test_extend_iterator_with_duplicates() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let dup_elems = vec![(1, 150), (2, 200), (1, 175)];
    map.extend(dup_elems.iter());
    let result_1 = map.get(&1);
    let result_2 = map.get(&2);
}

#[test]
fn test_extend_large_iterator() {
    let mut map = HashMap::new();
    let large_iter: Vec<(i32, i32)> = (0..1000).map(|x| (x, x * 10)).collect();
    map.extend(large_iter.iter());
}

#[test]
fn test_extend_existing_keys() {
    let mut map = HashMap::new();
    map.insert(5, 50);
    map.insert(10, 100);
    let existing_keys = vec![(5, 60), (10, 110), (15, 150)];
    map.extend(existing_keys.iter());
    let result_5 = map.get(&5);
    let result_10 = map.get(&10);
    let result_15 = map.get(&15);
}

