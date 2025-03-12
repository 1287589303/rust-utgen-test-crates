// Answer 0

#[test]
fn test_ldh_mask_b_in_0_to_9() {
    let mut accu = 0u128;
    let mut b = 0u8;

    while b <= 9 {
        if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _ = accu; // Placeholder to utilize 'accu' variable
}

#[test]
fn test_ldh_mask_b_equal_128() {
    let mut accu = 0u128;
    let mut b = 128u8;

    while b < 128 {
        if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    let _ = accu; // Placeholder to utilize 'accu' variable
}

