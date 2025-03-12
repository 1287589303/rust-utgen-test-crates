// Answer 0

#[test]
fn test_ldh_mask_b_127() {
    let b: u8 = 127;
    let mut accu = 0u128;
    if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
        accu |= 1u128 << b;
    }
}

#[test]
#[should_panic]
fn test_ldh_mask_b_128() {
    let b: u8 = 128;
    let mut accu = 0u128;
    if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
        accu |= 1u128 << b;
    }
}

