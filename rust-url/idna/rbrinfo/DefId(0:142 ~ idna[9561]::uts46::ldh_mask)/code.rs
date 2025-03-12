const fn ldh_mask() -> u128 {
    let mut accu = 0u128;
    let mut b = 0u8;
    while b < 128 {
        if !((b >= b'a' && b <= b'z') || (b >= b'0' && b <= b'9') || b == b'-' || b == b'.') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    accu
}