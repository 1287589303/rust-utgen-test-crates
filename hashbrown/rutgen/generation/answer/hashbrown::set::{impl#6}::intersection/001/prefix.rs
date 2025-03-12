// Answer 0

#[test]
fn intersection_empty_sets() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    let result = a.intersection(&b);
}

#[test]
fn intersection_self_empty() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let result = a.intersection(&b);
}

#[test]
fn intersection_equal_sizes() {
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
    let result = a.intersection(&b);
}

#[test]
fn intersection_large_equal_sizes() {
    let a: HashSet<i32> = (0..100).collect();
    let b: HashSet<i32> = (50..150).collect();
    let result = a.intersection(&b);
}

#[test]
fn intersection_single_element_equal() {
    let a: HashSet<i32> = [5].iter().cloned().collect();
    let b: HashSet<i32> = [5].iter().cloned().collect();
    let result = a.intersection(&b);
}

#[test]
fn intersection_multiple_equal() {
    let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4, 5].iter().cloned().collect();
    let result = a.intersection(&b);
}

