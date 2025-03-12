// Answer 0

#[test]
fn test_subtract_with_zero_bits() {
    let a = LookSet { bits: 0 };
    let b = LookSet { bits: 0 };
    let _result = a.subtract(b);
}

#[test]
fn test_subtract_empty_from_non_empty() {
    let a = LookSet { bits: 0b1100 };
    let b = LookSet { bits: 0 };
    let _result = a.subtract(b);
}

#[test]
fn test_subtract_non_empty_from_itself() {
    let a = LookSet { bits: 0b1100 };
    let b = LookSet { bits: 0b1100 };
    let _result = a.subtract(b);
}

#[test]
fn test_subtract_equal_non_empty_bits() {
    let a = LookSet { bits: 0b1111 };
    let b = LookSet { bits: 0b0111 };
    let _result = a.subtract(b);
}

#[test]
fn test_subtract_full_from_non_empty() {
    let a = LookSet { bits: 0b1111 };
    let b = LookSet { bits: 0xFFFFFFFF };
    let _result = a.subtract(b);
}

#[test]
fn test_subtract_zero_from_full() {
    let a = LookSet { bits: 0xFFFFFFFF };
    let b = LookSet { bits: 0 };
    let _result = a.subtract(b);
}

#[test]
fn test_subtract_max_u32_from_itself() {
    let a = LookSet { bits: 0xFFFFFFFF };
    let b = LookSet { bits: 0xFFFFFFFF };
    let _result = a.subtract(b);
}

