// Answer 0

#[test]
fn test_ldh_mask_b_less_than_128() {
    let result = ldh_mask();
}

#[test]
fn test_ldh_mask_b_equals_a() {
    let result = {
        let mut accu = 0u128;
        let mut b = b'a';
        if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
            accu |= 1u128 << b;
        }
        accu
    };
}

#[test]
fn test_ldh_mask_b_equals_z() {
    let result = {
        let mut accu = 0u128;
        let mut b = b'z';
        if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
            accu |= 1u128 << b;
        }
        accu
    };
}

#[test]
fn test_ldh_mask_b_equals_128() {
    let result = {
        let mut accu = 0u128;
        let mut b = 128;
        if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
            accu |= 1u128 << b;
        }
        accu
    };
}

