// Answer 0

#[test]
fn test_for_each_with_left_only() {
    let left_iter = vec![1, 2, 3].into_iter();
    let right_iter = std::iter::empty();
    let either = Either::Left(left_iter);
    
    either.for_each(|item| {
        // This closure runs for each item in the left iterator
        let _ = item + 1; // Example operation
    });
}

#[test]
fn test_for_each_with_right_only() {
    let left_iter = std::iter::empty();
    let right_iter = vec![4, 5, 6].into_iter();
    let either = Either::Right(right_iter);
    
    either.for_each(|item| {
        // This closure runs for each item in the right iterator
        let _ = item * 2; // Example operation
    });
}

#[test]
fn test_for_each_with_both_iterators() {
    let left_iter = vec![1, 2, 3].into_iter();
    let right_iter = vec![4, 5, 6].into_iter();
    let either = Either::Left(left_iter);
    
    either.for_each(|item| {
        let _ = item + 1; // Example operation for Left
    });

    let either = Either::Right(right_iter);
    
    either.for_each(|item| {
        let _ = item * 2; // Example operation for Right
    });
}

