// Answer 0

#[test]
fn test_extend_with_empty_iterator() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let empty_iter: Vec<(&i32, &i32)> = vec![];
    map.extend(empty_iter.iter());
}

#[test]
fn test_extend_with_single_element() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 100);
    let single_iter = vec![(&1, &1)];
    map.extend(single_iter.iter());
}

#[test]
fn test_extend_with_no_duplicate_keys() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 100);
    let non_duplicate_iter = vec![(&2, &2), (&3, &3)];
    map.extend(non_duplicate_iter.iter());
}

#[test]
fn test_extend_with_duplicate_keys() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 100);
    let duplicate_key_iter = vec![(&1, &1), (&2, &2)];
    map.extend(duplicate_key_iter.iter());
}

#[test]
fn test_extend_with_multiple_elements() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 100);
    let multiple_elements_iter = vec![
        (&1, &10), 
        (&2, &20), 
        (&3, &30),
        (&4, &40)
    ];
    map.extend(multiple_elements_iter.iter());
}

#[test]
fn test_extend_with_keys_in_range() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let keys_in_range_iter = (0..10).map(|k| (&k, &(k * 2)));
    map.extend(keys_in_range_iter);
}

#[test]
fn test_extend_with_negative_keys() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let negative_keys_iter = vec![(&-1, &1), (&-2, &2)];
    map.extend(negative_keys_iter.iter());
}

#[test]
fn test_extend_with_u64_keys() {
    let mut map: HashMap<u64, u32> = HashMap::new();
    let u64_iter = (0..10).map(|k| (&k, &(k as u32 * 2)));
    map.extend(u64_iter);
}

#[test]
fn test_extend_with_large_key_range() {
    let mut map: HashMap<i64, i32> = HashMap::new();
    let large_range_iter = (0..10).map(|k| (&(k as i64), &(k as i32)));
    map.extend(large_range_iter);
}

#[test]
fn test_extend_with_compatible_types() {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let compatible_iter = vec![(&0, &1), (&1, &2)];
    map.extend(compatible_iter.iter());
}

