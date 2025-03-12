// Answer 0

#[test]
fn test_bitxor_assign_with_empty_rhs() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_with_non_intersecting_rhs() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![4, 5, 6].into_iter().collect();
    a ^= &b;
}

#[test]
fn test_bitxor_assign_with_rhs_not_present_in_self() {
    let mut a: HashSet<String> = vec!["apple".to_string(), "banana".to_string()].into_iter().collect();
    let b: HashSet<String> = vec!["cherry".to_string(), "date".to_string()].into_iter().collect();
    a ^= &b;
}

