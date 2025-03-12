// Answer 0

#[test]
fn test_format_finite_f32() {
    struct Buffer {
        bytes: [u8; 64],
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 64] }
        }

        fn format_finite<F: Float>(&mut self, f: F) -> &str {
            unsafe {
                let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
                debug_assert!(n <= self.bytes.len());
                let slice = core::slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
                core::str::from_utf8_unchecked(slice)
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(3.14f32);
    assert_eq!(result, "3.14");
}

#[test]
fn test_format_finite_f64() {
    struct Buffer {
        bytes: [u8; 64],
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 64] }
        }

        fn format_finite<F: Float>(&mut self, f: F) -> &str {
            unsafe {
                let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
                debug_assert!(n <= self.bytes.len());
                let slice = core::slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
                core::str::from_utf8_unchecked(slice)
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(2.718281828459045f64);
    assert_eq!(result, "2.718281828459045");
}

#[test]
fn test_format_finite_zero() {
    struct Buffer {
        bytes: [u8; 64],
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { bytes: [0; 64] }
        }

        fn format_finite<F: Float>(&mut self, f: F) -> &str {
            unsafe {
                let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
                debug_assert!(n <= self.bytes.len());
                let slice = core::slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
                core::str::from_utf8_unchecked(slice)
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(0.0f32);
    assert_eq!(result, "0");
}

