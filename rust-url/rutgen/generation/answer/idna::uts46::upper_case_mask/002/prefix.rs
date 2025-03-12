// Answer 0

#[test]
fn test_upper_case_mask_with_b_at_A() {
    let mut accu = 0u128;
    let mut b = b'A';  // b == b'A'
    
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _result = accu; // Return value for b == b'A'
}

#[test]
fn test_upper_case_mask_with_b_at_Z_plus_one() {
    let mut accu = 0u128;
    let mut b = b'Z' + 1;  // b == b'['
    
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _result = accu; // Return value for b == b'['
}

#[test]
#[should_panic]
fn test_upper_case_mask_with_b_at_128() {
    let mut accu = 0u128;
    let mut b = 128;  // b == 128
    
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _result = accu; // This should not execute as b is not less than 128
}

