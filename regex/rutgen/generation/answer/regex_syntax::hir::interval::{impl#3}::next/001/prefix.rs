// Answer 0

#[test]
fn test_next_non_empty_slice_integers() {
    let values = vec![1, 2, 3];
    let mut iter = IntervalSetIter(values.iter());
    let _ = iter.next(); // should return Some(&1)
    let _ = iter.next(); // should return Some(&2)
    let _ = iter.next(); // should return Some(&3)
}

#[test]
fn test_next_one_element_slice() {
    let values = vec![42];
    let mut iter = IntervalSetIter(values.iter());
    let _ = iter.next(); // should return Some(&42)
}

#[test]
fn test_next_empty_slice() {
    let values: Vec<i32> = Vec::new();
    let mut iter = IntervalSetIter(values.iter());
    let _ = iter.next(); // should return None
}

#[test]
fn test_next_non_empty_slice_characters() {
    let values = vec!['a', 'b', 'c'];
    let mut iter = IntervalSetIter(values.iter());
    let _ = iter.next(); // should return Some(&'a')
    let _ = iter.next(); // should return Some(&'b')
    let _ = iter.next(); // should return Some(&'c')
}

#[test]
fn test_next_slice_with_repeated_elements() {
    let values = vec![1, 1, 2];
    let mut iter = IntervalSetIter(values.iter());
    let _ = iter.next(); // should return Some(&1)
    let _ = iter.next(); // should return Some(&1)
    let _ = iter.next(); // should return Some(&2)
}

#[test]
fn test_next_generic_slice() {
    let values: Vec<&str> = vec!["hello", "world"];
    let mut iter = IntervalSetIter(values.iter());
    let _ = iter.next(); // should return Some(&"hello")
    let _ = iter.next(); // should return Some(&"world")
}

