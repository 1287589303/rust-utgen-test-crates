// Answer 0

#[test]
fn test_extend_with_empty_iterator() {
    let mut set: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let empty_iter: Vec<&i32> = vec![];
    set.extend(empty_iter);
}

#[test]
fn test_extend_with_single_element_iterator() {
    let mut set: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let single_iter: Vec<&i32> = vec![&1];
    set.extend(single_iter);
}

#[test]
fn test_extend_with_multiple_elements_iterator() {
    let mut set: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let multiple_iter: Vec<&i32> = vec![&1, &2, &3, &4, &5];
    set.extend(multiple_iter);
}

#[test]
fn test_extend_with_duplicates() {
    let mut set: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let duplicate_iter: Vec<&i32> = vec![&1, &2, &2, &3, &3];
    set.extend(duplicate_iter);
}

#[test]
fn test_extend_with_large_iterator() {
    let mut set: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    let large_iter: Vec<&i32> = (0..1000).map(|x| &x).collect();
    set.extend(large_iter);
}

