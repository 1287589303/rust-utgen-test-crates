// Answer 0

#[test]
fn test_symmetric_difference_empty_sets() {
    let set_a: HashSet<i32, DefaultHashBuilder> = HashSet::new();
    let set_b: HashSet<i32, DefaultHashBuilder> = HashSet::new();
    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };
    let _result: Vec<_> = symmetric_difference.collect();
}

#[test]
fn test_symmetric_difference_identical_sets() {
    let set_a: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![1, 2, 3]);
    let set_b: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![1, 2, 3]);
    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };
    let _result: Vec<_> = symmetric_difference.collect();
}

#[test]
fn test_symmetric_difference_completely_different_sets() {
    let set_a: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![1, 2, 3]);
    let set_b: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![4, 5, 6]);
    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };
    let _result: Vec<_> = symmetric_difference.collect();
}

#[test]
fn test_symmetric_difference_partially_overlapping_sets() {
    let set_a: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![1, 2, 3]);
    let set_b: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![2, 3, 4]);
    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };
    let _result: Vec<_> = symmetric_difference.collect();
}

#[test]
fn test_symmetric_difference_one_empty_set() {
    let set_a: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(vec![1, 2, 3]);
    let set_b: HashSet<i32, DefaultHashBuilder> = HashSet::new();
    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };
    let _result: Vec<_> = symmetric_difference.collect();
}

#[test]
fn test_symmetric_difference_large_sets() {
    let set_a: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(1..1000);
    let set_b: HashSet<i32, DefaultHashBuilder> = HashSet::from_iter(500..1500);
    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };
    let _result: Vec<_> = symmetric_difference.collect();
}

