// Answer 0

#[test]
fn test_format64_zero() {
    let f: f64 = 0.0;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_lossy(&buffer[..len]);
        assert_eq!(result, "0.0");
    }
}

#[test]
fn test_format64_negative_zero() {
    let f: f64 = -0.0;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_lossy(&buffer[..len]);
        assert_eq!(result, "-0.0");
    }
}

#[test]
fn test_format64_small_negative() {
    let f: f64 = -1e-324;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_lossy(&buffer[..len]);
        assert_eq!(result, "0.000000000000000000000001");
    }
}

#[test]
fn test_format64_small_positive() {
    let f: f64 = 1e-324;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = format64(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_lossy(&buffer[..len]);
        assert_eq!(result, "0.000000000000000000000001");
    }
}

