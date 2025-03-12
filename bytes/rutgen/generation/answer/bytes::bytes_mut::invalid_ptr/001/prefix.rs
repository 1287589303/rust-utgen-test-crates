// Answer 0

#[test]
fn test_invalid_ptr_zero() {
    let addr: usize = 0;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_min_positive() {
    let addr: usize = 1;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_max() {
    let addr: usize = (1 << PTR_WIDTH) - 1;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_mid_range() {
    let addr: usize = (1 << (PTR_WIDTH - 1)); 
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

