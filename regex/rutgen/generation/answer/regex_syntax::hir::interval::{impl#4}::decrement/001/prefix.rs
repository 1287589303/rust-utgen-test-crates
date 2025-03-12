// Answer 0

#[test]
fn test_decrement_at_zero() {
    let input: u8 = 0;
    let result = input.decrement();
}

#[test]
fn test_decrement_at_one() {
    let input: u8 = 1;
    let result = input.decrement();
}

#[test]
fn test_decrement_at_max_value() {
    let input: u8 = 255;
    let result = input.decrement();
}

#[test]
#[should_panic]
fn test_decrement_below_zero() {
    let input: u8 = 0;
    let result = input.decrement();
}

