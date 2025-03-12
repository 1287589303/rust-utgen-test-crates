// Answer 0

#[test]
fn test_get_non_existent_value_empty_hashset() {
    let set: HashSet<i32> = HashSet::default();
    let result = set.get(&4);
}

#[test]
fn test_get_non_existent_value_non_empty_hashset() {
    let mut set: HashSet<i32> = HashSet::default();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let result = set.get(&4);
}

