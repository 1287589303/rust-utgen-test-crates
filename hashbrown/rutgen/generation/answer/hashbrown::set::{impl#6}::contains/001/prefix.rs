// Answer 0

#[test]
fn test_contains_with_existing_element() {
    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let _ = set.contains(&1);
}

#[test]
fn test_contains_with_non_existing_element() {
    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let _ = set.contains(&4);
}

#[test]
fn test_contains_with_empty_set() {
    let set: HashSet<i32> = HashSet::new();
    let _ = set.contains(&1);
}

#[test]
fn test_contains_with_single_element_set_existing() {
    let set: HashSet<i32> = [5].iter().cloned().collect();
    let _ = set.contains(&5);
}

#[test]
fn test_contains_with_single_element_set_non_existing() {
    let set: HashSet<i32> = [5].iter().cloned().collect();
    let _ = set.contains(&6);
}

