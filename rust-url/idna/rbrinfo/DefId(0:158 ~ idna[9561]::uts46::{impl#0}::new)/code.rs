pub const fn new(deny_glyphless: bool, deny_list: &str) -> Self {
        let mut bits = UPPER_CASE_MASK;
        if deny_glyphless {
            bits |= GLYPHLESS_MASK;
        }
        let mut i = 0;
        let bytes = deny_list.as_bytes();
        while i < bytes.len() {
            let b = bytes[i];
            assert!(b < 0x80, "ASCII deny list must be ASCII.");
            // assert_ne not yet available in const context.
            assert!(b != b'.', "ASCII deny list must not contain the dot.");
            assert!(b != b'-', "ASCII deny list must not contain the hyphen.");
            assert!(
                !((b >= b'0') && (b <= b'9')),
                "ASCII deny list must not contain digits."
            );
            assert!(
                !((b >= b'a') && (b <= b'z')),
                "ASCII deny list must not contain letters."
            );
            assert!(
                !((b >= b'A') && (b <= b'Z')),
                "ASCII deny list must not contain letters."
            );
            bits |= 1u128 << b;
            i += 1;
        }
        AsciiDenyList { bits }
    }