// Answer 0

#[test]
fn test_factor_iter_mut_with_non_empty_right_vector() {
    let mut right: Either<Vec<&str>, Vec<i32>> = Right(vec![0, 1, 2]);
    right.factor_iter_mut().for_each(|x| {
        if let Right(r) = x {
            *r = -*r;
        }
    });
}

#[test]
fn test_factor_iter_mut_with_empty_right_vector() {
    let mut right: Either<Vec<&str>, Vec<i32>> = Right(vec![]);
    right.factor_iter_mut().for_each(|x| {
        if let Right(r) = x {
            *r = -*r;
        }
    });
}

#[test]
fn test_factor_iter_mut_with_right_array() {
    let mut right: Either<Vec<&str>, [&mut i32; 3]> = Right([&mut 0, &mut 1, &mut 2]);
    right.factor_iter_mut().for_each(|x| {
        if let Right(r) = x {
            *r = -*r;
        }
    });
}

#[test]
fn test_factor_iter_mut_with_large_right_vector() {
    let mut right: Either<Vec<&str>, Vec<i32>> = Right(vec![3, 4, 5, 6, 7]);
    right.factor_iter_mut().for_each(|x| {
        if let Right(r) = x {
            *r = -*r;
        }
    });
}

