// Answer 0

#[test]
fn test_all_with_empty_left_and_non_empty_right_all_true() {
    let right_iter = vec![1, 2, 3].into_iter();
    let inner = Either::Right(right_iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.all(|&x| x > 0);
}

#[test]
fn test_all_with_empty_left_and_non_empty_right_some_false() {
    let right_iter = vec![1, 2, 3, 4].into_iter();
    let inner = Either::Right(right_iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.all(|&x| x < 3);
}

#[test]
fn test_all_with_empty_left_and_non_empty_right_empty_condition() {
    let right_iter = vec![5, 6, 7].into_iter();
    let inner = Either::Right(right_iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.all(|&_| false);
}

