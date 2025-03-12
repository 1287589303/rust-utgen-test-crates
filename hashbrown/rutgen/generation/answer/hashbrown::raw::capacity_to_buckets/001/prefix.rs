// Answer 0

#[test]
fn test_capacity_to_buckets_below_four() {
    let cap: usize = 1;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(4);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_three() {
    let cap: usize = 3;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(4);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_four() {
    let cap: usize = 4;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(8);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_five() {
    let cap: usize = 5;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(8);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_six() {
    let cap: usize = 6;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(8);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_seven() {
    let cap: usize = 7;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(8);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_eight() {
    let cap: usize = 8;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(16);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_sixteen() {
    let cap: usize = 16;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(32);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_thirty_two() {
    let cap: usize = 32;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(64);
    assert_eq!(result, expected);
}

#[test]
fn test_capacity_to_buckets_sixty_four() {
    let cap: usize = 64;
    let result = capacity_to_buckets(cap);

    let expected: Option<usize> = Some(128);
    assert_eq!(result, expected);
}

