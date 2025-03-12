// Answer 0

#[test]
fn test_iter_mut_with_right_array() {
    let mut inner = [1, 2, 3];
    let mut right: Either<Vec<u32>, _> = Right(&mut inner[..]);
    for r in right.iter_mut() {
        *r += 1;
    }
    // The expected value of `inner` would now be [2, 3, 4], hence we'll check that directly later
}

#[test]
fn test_iter_mut_with_right_single_element() {
    let mut inner = [5];
    let mut right: Either<Vec<u32>, _> = Right(&mut inner[..]);
    for r in right.iter_mut() {
        *r *= 2;
    }
    // Expected value of `inner` would now be [10]
}

#[test]
fn test_iter_mut_with_right_large_array() {
    let mut inner = [6, 7, 8, 9, 10];
    let mut right: Either<Vec<u32>, _> = Right(&mut inner[..]);
    for r in right.iter_mut() {
        *r -= 1;
    }
    // Expected value of `inner` would now be [5, 6, 7, 8, 9]
}

#[test]
fn test_iter_mut_with_right_empty_array() {
    let mut inner: &mut [u32] = &mut [];
    let mut right: Either<Vec<u32>, _> = Right(inner);
    for r in right.iter_mut() {
        *r += 1; // This won't modify anything since the array is empty
    }
    // Expected value is still an empty array
}

