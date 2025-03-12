// Answer 0

#[test]
fn test_sub_assign_rhs_len_less_than_self_len() {
    let mut self_set: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let rhs_set: HashSet<i32> = vec![3, 4].into_iter().collect();
    
    self_set -= &rhs_set;
    
    let mut i = 0;
    let expected = [1, 2, 5];
    for x in &self_set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_rhs_contains_items_in_self() {
    let mut self_set: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let rhs_set: HashSet<i32> = vec![3, 4].into_iter().collect();
    
    self_set -= &rhs_set;
    
    let mut i = 0;
    let expected = [1, 2, 5];
    for x in &self_set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_rhs_has_items_not_in_self() {
    let mut self_set: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let rhs_set: HashSet<i32> = vec![4, 5].into_iter().collect();
    
    self_set -= &rhs_set;

    let mut i = 0;
    let expected = [1, 2, 3]; // No change should occur
    for x in &self_set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

