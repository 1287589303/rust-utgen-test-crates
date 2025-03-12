// Answer 0

#[test]
fn test_intersection_non_empty() {
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
    let result = &a & &b;
}

#[test]
fn test_intersection_all_common() {
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let result = &a & &b;
}

#[test]
fn test_intersection_no_common() {
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [4, 5, 6].iter().cloned().collect();
    let result = &a & &b;
}

#[test]
fn test_intersection_single_common() {
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [3].iter().cloned().collect();
    let result = &a & &b;
}

#[test]
fn test_intersection_same_single_element() {
    let a: HashSet<i32> = [5].iter().cloned().collect();
    let b: HashSet<i32> = [5].iter().cloned().collect();
    let result = &a & &b;
}

