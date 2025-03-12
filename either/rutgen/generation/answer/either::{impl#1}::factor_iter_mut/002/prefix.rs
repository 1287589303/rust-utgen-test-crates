// Answer 0

#[test]
fn test_factor_iter_mut_left_single_item() {
    let mut left: Either<&mut [&str], Vec<u8>> = Left(&mut ["hello"]);
    left.factor_iter_mut().for_each(|x| *x.unwrap_left() = "goodbye");
}

#[test]
fn test_factor_iter_mut_left_multiple_items() {
    let mut left: Either<&mut [&str; 3], Vec<u8>> = Left(&mut ["one", "two", "three"]);
    left.factor_iter_mut().for_each(|x| *x.unwrap_left() = "changed");
}

#[test]
fn test_factor_iter_mut_left_max_items() {
    let mut left: Either<&mut Vec<u32>, Vec<u8>> = Left(&mut vec![1; 1000]);
    left.factor_iter_mut().for_each(|x| *x.unwrap_left() += 1);
} 

#[test]
#[should_panic]
fn test_factor_iter_mut_right_should_panic() {
    let mut right: Either<&str, Vec<u8>> = Right(vec![0, 1, 2]);
    right.factor_iter_mut(); // This should panic as we expect Left
}

