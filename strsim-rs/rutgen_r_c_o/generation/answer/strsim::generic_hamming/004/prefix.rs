// Answer 0

#[test]
fn test_empty_second_iterator() {
    let first = vec![1, 2, 3];
    let second: Vec<i32> = vec![];
    let result = generic_hamming(first, second);
}

#[test]
fn test_first_iterator_longer() {
    let first = vec![1, 2, 3];
    let second = vec![1, 2];
    let result = generic_hamming(first, second);
}

#[test]
fn test_first_iterator_mismatch() {
    let first = vec![1, 2, 3];
    let second = vec![1, 2, 4];
    let result = generic_hamming(first, second);
}

