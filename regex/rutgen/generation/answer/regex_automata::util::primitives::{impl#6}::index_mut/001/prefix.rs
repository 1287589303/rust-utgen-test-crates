// Answer 0

#[test]
fn test_index_mut_zero() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let index = SmallIndex::ZERO;
    let _result = vec.index_mut(index);
}

#[test]
fn test_index_mut_mid() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let index = SmallIndex::new(2).unwrap(); // assuming `new` is a valid way to create a SmallIndex
    let _result = vec.index_mut(index);
}

#[test]
fn test_index_mut_max() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let index = SmallIndex::MAX;
    let _result = vec.index_mut(index);
} 

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds() {
    let mut vec = vec![1, 2, 3];
    let index = SmallIndex::new(3).unwrap(); // Next index, should panic since it exceeds the length
    let _result = vec.index_mut(index);
}

