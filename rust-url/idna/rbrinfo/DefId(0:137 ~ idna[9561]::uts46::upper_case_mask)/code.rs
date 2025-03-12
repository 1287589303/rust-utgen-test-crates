const fn upper_case_mask() -> u128 {
    let mut accu = 0u128;
    let mut b = 0u8;
    while b < 128 {
        if (b >= b'A') && (b <= b'Z') {
            accu |= 1u128 << b;
        }
        b += 1;
    }
    accu
}