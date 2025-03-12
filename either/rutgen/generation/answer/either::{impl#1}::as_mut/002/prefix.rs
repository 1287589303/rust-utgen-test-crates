// Answer 0

#[test]
fn test_as_mut_left() {
    let mut left = Left(123);
    let result = left.as_mut();
    if let Either::Left(l) = result {
        *l = 999;
    }
}

#[test]
fn test_as_mut_right() {
    let mut right = Right(123);
    let result = right.as_mut();
    if let Either::Right(r) = result {
        // This should not mutate the right value
        *r = 999; // This will be effectively ignored since it's a Right
    }
}

#[test]
fn test_as_mut_left_zero_boundary() {
    let mut left = Left(0u32);
    let result = left.as_mut();
    if let Either::Left(l) = result {
        *l = 999;
    }
}

#[test]
fn test_as_mut_left_higher_boundary() {
    let mut left = Left(999u32);
    let result = left.as_mut();
    if let Either::Left(l) = result {
        *l = 999;
    }
}

#[test]
fn test_as_mut_right_lower_boundary() {
    let mut right = Right(0u32);
    let result = right.as_mut();
    if let Either::Right(r) = result {
        // This should not mutate the right value
        *r = 999; // This will be effectively ignored since it's a Right
    }
}

#[test]
fn test_as_mut_right_higher_boundary() {
    let mut right = Right(999u32);
    let result = right.as_mut();
    if let Either::Right(r) = result {
        // This should not mutate the right value
        *r = 999; // This will be effectively ignored since it's a Right
    }
}

