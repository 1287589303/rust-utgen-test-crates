// Answer 0

#[test]
fn test_extend_right_with_empty_iterator() {
    let mut either = Either::Right(vec![1, 2, 3]);
    let empty_iter: Vec<i32> = vec![];
    either.extend(empty_iter);
}

#[test]
fn test_extend_right_with_single_item_iterator() {
    let mut either = Either::Right(vec![1, 2, 3]);
    let single_item_iter = vec![4];
    either.extend(single_item_iter);
}

#[test]
fn test_extend_right_with_multiple_items_iterator() {
    let mut either = Either::Right(vec![1, 2, 3]);
    let multiple_items_iter = vec![4, 5, 6];
    either.extend(multiple_items_iter);
}

#[test]
fn test_extend_right_with_large_iterator() {
    let mut either = Either::Right(vec![1, 2, 3]);
    let large_iter = (7..=1000).collect::<Vec<i32>>();
    either.extend(large_iter);
}

#[test]
fn test_extend_right_with_varied_types() {
    let mut either = Either::Right(vec!["A", "B"]);
    let varied_iter = vec!["C", "D", "E"];
    either.extend(varied_iter);
}

