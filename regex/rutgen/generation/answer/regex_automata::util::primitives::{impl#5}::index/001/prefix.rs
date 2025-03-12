// Answer 0

#[test]
fn test_index_valid_zero() {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let index = SmallIndex::ZERO;
    let result = data.index(index);
}

#[test]
fn test_index_valid_max() {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let index = SmallIndex::new_unchecked(data.len() - 1);
    let result = data.index(index);
}

#[test]
fn test_index_valid_mid() {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let index = SmallIndex::new_unchecked(2);
    let result = data.index(index);
}

#[test]
#[should_panic]
fn test_index_invalid_too_large() {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let index = SmallIndex::new_unchecked(data.len());
    let result = data.index(index);
}

#[test]
#[should_panic]
fn test_index_invalid_negative() {
    let data: Vec<i32> = vec![10, 20, 30, 40];
    let index = SmallIndex::new_unchecked(usize::MAX);
    let result = data.index(index);
}

