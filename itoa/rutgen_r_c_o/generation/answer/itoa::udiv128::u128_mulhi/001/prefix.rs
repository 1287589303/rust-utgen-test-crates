// Answer 0

#[test]
fn test_u128_mulhi_with_zero() {
    let x: u128 = 0;
    let y: u128 = 0;
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_one() {
    let x: u128 = 1;
    let y: u128 = 1;
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_max_value() {
    let x: u128 = 340282366920938463463374607431768211455; // max u128
    let y: u128 = 340282366920938463463374607431768211455; // max u128
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_half_max_value() {
    let x: u128 = 170141183460469231731687303715884105727; // half of max u128
    let y: u128 = 170141183460469231731687303715884105727; // half of max u128
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_max_first_operand() {
    let x: u128 = 340282366920938463463374607431768211455; // max u128
    let y: u128 = 1;
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_max_second_operand() {
    let x: u128 = 1;
    let y: u128 = 340282366920938463463374607431768211455; // max u128
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_large_first_operand() {
    let x: u128 = 170141183460469231731687303715884105728; // slightly more than half
    let y: u128 = 2;
    let _ = u128_mulhi(x, y);
}

#[test]
fn test_u128_mulhi_with_large_second_operand() {
    let x: u128 = 2;
    let y: u128 = 170141183460469231731687303715884105728; // slightly more than half
    let _ = u128_mulhi(x, y);
}

