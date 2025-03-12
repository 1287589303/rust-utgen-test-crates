// Answer 0

#[test]
fn test_format64_sign_false_ieee_exponent_zero_k_neg_325() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = f64::from_bits(0b1000000000000000000000000000000000000000000000000000000000000001); // Arbitrary bit pattern for negative number
    assert!(!f.is_finite()); // Ensure the value isn't finite

    unsafe {
        let mut buffer: [MaybeUninit<u8>; 24] = MaybeUninit::uninit().assume_init();
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_sign_false_ieee_exponent_zero_k_neg_324() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = f64::from_bits(0b0000000000000000000000000000000000000000000000000000000000000000); // Bit pattern for zero
    assert!(f.is_finite()); // Ensure the value is finite

    unsafe {
        let mut buffer: [MaybeUninit<u8>; 24] = MaybeUninit::uninit().assume_init();
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

