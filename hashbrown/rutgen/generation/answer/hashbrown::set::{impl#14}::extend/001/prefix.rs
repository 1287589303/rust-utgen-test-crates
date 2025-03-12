// Answer 0

#[test]
fn test_extend_non_empty_iterator() {
    let mut set: crate::HashSet<i32> = crate::HashSet::default();
    let values = vec![1, 2, 3, 4];
    set.extend(&values);
}

#[test]
fn test_extend_empty_iterator() {
    let mut set: crate::HashSet<i32> = crate::HashSet::default();
    let values: Vec<i32> = vec![];
    set.extend(&values);
}

#[test]
fn test_extend_max_size_iterator() {
    let mut set: crate::HashSet<i32> = crate::HashSet::default();
    let max_size = usize::MAX; 
    let values: Vec<i32> = (0..max_size).map(|x| x as i32).collect();
    set.extend(&values);
}

#[test]
fn test_extend_single_element_iterator() {
    let mut set: crate::HashSet<i32> = crate::HashSet::default();
    let values = vec![42];
    set.extend(&values);
}

#[test]
fn test_extend_large_size_iterator() {
    let mut set: crate::HashSet<i32> = crate::HashSet::default();
    let values: Vec<i32> = (0..1000).map(|x| x as i32).collect();
    set.extend(&values);
}

#[test]
fn test_extend_with_duplicates() {
    let mut set: crate::HashSet<i32> = crate::HashSet::default();
    let values = vec![1, 2, 2, 3, 3, 3, 4];
    set.extend(&values);
}

