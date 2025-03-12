// Answer 0

#[test]
fn test_bitor_assign_with_non_overlapping_sets() {
    use hashbrown::HashSet;
    
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![4, 5, 6].into_iter().collect();

    a |= &b;
}

#[test]
fn test_bitor_assign_with_partial_overlapping_sets() {
    use hashbrown::HashSet;
    
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    a |= &b;
}

#[test]
fn test_bitor_assign_with_empty_rhs() {
    use hashbrown::HashSet;
    
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();

    a |= &b;
}

