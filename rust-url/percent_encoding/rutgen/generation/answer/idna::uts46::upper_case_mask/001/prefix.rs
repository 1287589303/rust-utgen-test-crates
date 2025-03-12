// Answer 0

#[test]
fn test_upper_case_mask_a() {
    let mut accu = 0u128;
    let mut b = 65; // 'A'
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _ = accu; // Use this value to ensure the function is called.
}

#[test]
fn test_upper_case_mask_z() {
    let mut accu = 0u128;
    let mut b = 90; // 'Z'
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _ = accu; // Use this value to ensure the function is called.
}

#[test]
fn test_upper_case_mask_exceed() {
    let mut accu = 0u128;
    let mut b = 128; // exceeds loop bound
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _ = accu; // Use this value to ensure the function is called.
}

