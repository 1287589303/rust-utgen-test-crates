// Answer 0

#[test]
fn test_into_iter_right_with_vec() {
    let right: Either<Vec<u32>, Vec<u32>> = Right(vec![1, 2, 3]);
    let _result: Either<std::vec::IntoIter<u32>, std::vec::IntoIter<u32>> = right.into_iter();
}

#[test]
fn test_into_iter_right_with_array() {
    let right: Either<[u32; 3], Vec<u32>> = Right([1, 2, 3]);
    let _result: Either<std::array::IntoIter<u32, 3>, std::vec::IntoIter<u32>> = right.into_iter();
}

#[test]
fn test_into_iter_right_with_string() {
    let right: Either<String, Vec<u32>> = Right(String::from("test"));
    let _result: Either<std::string::IntoIter, std::vec::IntoIter<u32>> = right.into_iter();
}

#[test]
fn test_into_iter_right_with_slice() {
    let right: Either<&[u32], Vec<u32>> = Right(&[1, 2, 3]);
    let _result: Either<std::slice::Iter<u32>, std::vec::IntoIter<u32>> = right.into_iter();
}

