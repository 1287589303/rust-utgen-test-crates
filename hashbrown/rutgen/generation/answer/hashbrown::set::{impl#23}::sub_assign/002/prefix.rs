// Answer 0

#[test]
fn test_sub_assign_with_larger_self() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<i32> = vec![6, 7, 8].into_iter().collect();
    
    a -= &b;
}

#[test]
fn test_sub_assign_remains_unaffected() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = vec![5, 6].into_iter().collect();
    
    a -= &b;
}

#[test]
fn test_sub_assign_with_multiple_elements() {
    let mut a: HashSet<i32> = vec![10, 20, 30, 40, 50, 60].into_iter().collect();
    let b: HashSet<i32> = vec![70, 80].into_iter().collect();
    
    a -= &b;
}

#[test]
fn test_sub_assign_empty_rhs() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();
    
    a -= &b;
}

