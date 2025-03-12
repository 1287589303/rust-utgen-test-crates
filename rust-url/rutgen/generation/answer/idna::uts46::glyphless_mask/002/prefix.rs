// Answer 0

#[test]
fn test_glyphless_mask_with_b_less_than_128_and_condition_false() {
    const fn glyphless_mask() -> u128 {
        let mut accu = 0u128;
        let mut b = 65; // Arbitrary value in the range (0, 127)
        while b < 128 {
            if (b <= b' ') || (b == 0x7F) {
                accu |= 1u128 << b;
            }
            b += 1;
        }
        accu
    }
    let _ = glyphless_mask();
}

#[test]
fn test_glyphless_mask_with_b_equal_128() {
    const fn glyphless_mask() -> u128 {
        let mut accu = 0u128;
        let mut b = 128; // Boundary case
        while b < 128 {
            if (b <= b' ') || (b == 0x7F) {
                accu |= 1u128 << b;
            }
            b += 1;
        }
        accu
    }
    let _ = glyphless_mask();
}

