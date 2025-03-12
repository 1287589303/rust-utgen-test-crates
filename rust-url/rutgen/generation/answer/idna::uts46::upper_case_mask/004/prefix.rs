// Answer 0

#[test]
fn test_upper_case_mask_b_equal_128() {
    let b = 128;
    let mut accu = 0u128;
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _result = accu; // Call the function indirectly by using the variable
}

#[test]
#[should_panic]
fn test_upper_case_mask_b_exceeds_128() {
    let b = 129; // Exceeding the range
    let mut accu = 0u128;
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _result = accu; // Call the function indirectly by using the variable
}

