// Answer 0

#[test]
fn test_bitand_assign_non_empty_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_with_all_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_no_common_elements() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_one_element_common() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_edge_case_a_subsets_b() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_edge_case_b_subsets_a() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<_> = vec![3, 4].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_different_sizes_common() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 2, 3, 5].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    a &= &b;
}

#[test]
fn test_bitand_assign_identical_sets() {
    use hashbrown::HashSet;

    let mut a: HashSet<_> = vec![1, 1, 2, 2, 3, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    a &= &b;
}

