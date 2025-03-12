// Answer 0

#[test]
fn test_as_usize_valid_negative_one() {
    let value: i32 = -1;
    let _result = value.as_usize();
}

#[test]
fn test_as_usize_valid_zero() {
    let value: i32 = 0;
    let _result = value.as_usize();
}

#[test]
fn test_as_usize_valid_one() {
    let value: i32 = 1;
    let _result = value.as_usize();
}

#[test]
fn test_as_usize_valid_two() {
    let value: i32 = 2;
    let _result = value.as_usize();
}

#[test]
fn test_as_usize_valid_max() {
    let value: i32 = 2_147_483_647;
    let _result = value.as_usize();
}

#[should_panic]
#[test]
fn test_as_usize_overflow_below_min() {
    let value: i32 = -2_147_483_649;
    let _result = value.as_usize();
}

#[should_panic]
#[test]
fn test_as_usize_overflow_above_max() {
    let value: i32 = 2_147_483_648;
    let _result = value.as_usize();
}

