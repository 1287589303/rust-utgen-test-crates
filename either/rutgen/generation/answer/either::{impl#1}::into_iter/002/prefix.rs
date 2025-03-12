// Answer 0

#[test]
fn test_into_iter_left_vector_to_vector() {
    let left: Either<Vec<u32>, Vec<String>> = Left(vec![1, 2, 3, 4, 5]);
    let result: Either<Vec<u32>::IntoIter, Vec<String>::IntoIter> = left.into_iter();
}

#[test]
fn test_into_iter_left_array_to_vector() {
    let left: Either<[u32; 5], Vec<String>> = Left([1, 2, 3, 4, 5]);
    let result: Either<impl Iterator<Item = u32>, Vec<String>::IntoIter> = left.into_iter();
}

#[test]
fn test_into_iter_left_vec_to_empty_right() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![10, 20, 30]);
    let result: Either<Vec<u32>::IntoIter, Vec<u32>::IntoIter> = left.into_iter();
}

#[test]
fn test_into_iter_left_empty_vec_to_vector() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![]);
    let result: Either<Vec<u32>::IntoIter, Vec<u32>::IntoIter> = left.into_iter();
}

#[test]
fn test_into_iter_left_single_element_vector_to_vector() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![100]);
    let result: Either<Vec<u32>::IntoIter, Vec<u32>::IntoIter> = left.into_iter();
}

