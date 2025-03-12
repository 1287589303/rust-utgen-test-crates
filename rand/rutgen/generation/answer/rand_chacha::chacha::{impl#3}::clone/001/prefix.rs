// Answer 0

#[test]
fn test_clone_default_u8() {
    let original = Array64([0u8; 64]);
    let cloned = original.clone();
}

#[test]
fn test_clone_default_i32() {
    let original = Array64([0i32; 64]);
    let cloned = original.clone();
}

#[test]
fn test_clone_default_f64() {
    let original = Array64([0.0f64; 64]);
    let cloned = original.clone();
}

#[test]
fn test_clone_initialized_u8() {
    let original = Array64([1u8; 64]);
    let cloned = original.clone();
}

#[test]
fn test_clone_initialized_i32() {
    let original = Array64([2i32; 64]);
    let cloned = original.clone();
}

#[test]
fn test_clone_initialized_f64() {
    let original = Array64([3.0f64; 64]);
    let cloned = original.clone();
}

