// Answer 0

#[test]
fn test_format32_case_sign_true_ieee_exponent_zero_k_neg() {
    unsafe {
        let f: f32 = -0.0; // sign is true

        let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = format32(f, buffer.as_mut_ptr() as *mut u8);

        let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result_str = core::str::from_utf8_unchecked(slice);
        assert_eq!(result_str, "-0.0");
    }
}

#[test]
fn test_format32_case_sign_true_ieee_exponent_non_zero() {
    unsafe {
        let f: f32 = -1.0; // sign is true

        let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = format32(f, buffer.as_mut_ptr() as *mut u8);

        let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let result_str = core::str::from_utf8_unchecked(slice);
        assert_eq!(result_str, "-1.0");
    }
}

#[test]
fn test_format32_case_sign_true_ieee_exponent_zero_k_negative_boundary() {
    unsafe {
        let f: f32 = -1.175494e-38; // sign is true (k will lead to -45)

        let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = format32(f, buffer.as_mut_ptr() as *mut u8);

        let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        // expectation based on how the function handles low magnitude negatives
        assert!(len > 0);
    }
}

