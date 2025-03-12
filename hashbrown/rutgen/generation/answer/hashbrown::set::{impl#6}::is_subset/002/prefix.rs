// Answer 0

#[test]
fn test_is_subset_self_larger_with_non_matching_values() {
    let larger_set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut smaller_set = HashSet::new();
    smaller_set.insert(4);
    smaller_set.insert(5);
    smaller_set.insert(6);
    assert_eq!(smaller_set.is_subset(&larger_set), false);
}

#[test]
fn test_is_subset_self_empty_other_non_empty() {
    let other_set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let empty_set: HashSet<i32> = HashSet::new();
    assert_eq!(empty_set.is_subset(&other_set), true);
}

#[test]
fn test_is_subset_self_non_empty_other_empty() {
    let other_set: HashSet<i32> = HashSet::new();
    let mut non_empty_set = HashSet::new();
    non_empty_set.insert(1);
    assert_eq!(non_empty_set.is_subset(&other_set), false);
}

