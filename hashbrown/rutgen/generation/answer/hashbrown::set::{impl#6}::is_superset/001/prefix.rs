// Answer 0

#[test]
fn test_is_superset_empty_set() {
    let empty: HashSet<i32> = HashSet::new();
    let sub: HashSet<i32> = [1, 2].iter().cloned().collect();
    assert!(empty.is_superset(&sub) == false);
}

#[test]
fn test_is_superset_within_set() {
    let sub: HashSet<i32> = [1, 2].iter().cloned().collect();
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(1);
    assert!(set.is_superset(&sub) == false);
}

#[test]
fn test_is_superset_equal_set() {
    let sub: HashSet<i32> = [1, 2].iter().cloned().collect();
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    assert!(set.is_superset(&sub) == true);
}

#[test]
fn test_is_superset_with_extra_element() {
    let sub: HashSet<i32> = [1, 2].iter().cloned().collect();
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);
    assert!(set.is_superset(&sub) == true);
}

#[test]
fn test_is_superset_disjoint_set() {
    let sub: HashSet<i32> = [3, 4].iter().cloned().collect();
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    assert!(set.is_superset(&sub) == false);
}

#[test]
fn test_is_superset_empty_sub() {
    let empty: HashSet<i32> = HashSet::new();
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    assert!(set.is_superset(&empty) == true);
}

