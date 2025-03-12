// Answer 0

#[test]
fn test_is_subset_empty_set() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = HashSet::new();

    set.is_subset(&sup);
}

#[test]
fn test_is_subset_identical_sets() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = [1, 2, 3].into_iter().collect();

    set.is_subset(&sup);
}

#[test]
fn test_is_subset_with_subset_elements() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = [1, 2].into_iter().collect();

    set.is_subset(&sup);
}

#[test]
fn test_is_subset_same_elements_different_order() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = [2, 1].into_iter().collect();

    set.is_subset(&sup);
}

#[test]
fn test_is_subset_with_additional_elements() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = [1, 2, 4].into_iter().collect();

    set.is_subset(&sup);
}

