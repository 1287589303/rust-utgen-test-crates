// Answer 0

#[test]
fn test_index_mut_zero() {
    let mut data = [0u8; 10];
    let index = SmallIndex::new_unchecked(0);
    let result = data.index_mut(index);
}

#[test]
fn test_index_mut_high() {
    let mut data = [0u8; 10];
    let index = SmallIndex::new_unchecked(SmallIndex::LIMIT - 1);
    let result = data.index_mut(index);
}

#[test]
fn test_index_mut_middle() {
    let mut data = [0u8; 10];
    let index = SmallIndex::new_unchecked(5);
    let result = data.index_mut(index);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_low() {
    let mut data = [0u8; 10];
    let index = SmallIndex::new_unchecked(usize::MAX);
    let _result = data.index_mut(index);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_high() {
    let mut data = [0u8; 10];
    let index = SmallIndex::new_unchecked(SmallIndex::LIMIT);
    let _result = data.index_mut(index);
}

