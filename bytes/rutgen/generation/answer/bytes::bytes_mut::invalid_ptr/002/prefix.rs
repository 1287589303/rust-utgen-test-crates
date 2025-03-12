// Answer 0

#[test]
fn test_invalid_ptr_zero() {
    let addr: usize = 0;
    let result = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_one() {
    let addr: usize = 1;
    let result = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_max_original_capacity_width_plus_one() {
    let addr: usize = MAX_ORIGINAL_CAPACITY_WIDTH + 1;
    let result = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_max_vec_pos_plus_one() {
    let addr: usize = MAX_VEC_POS + 1;
    let result = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_usize_max() {
    let addr: usize = usize::MAX;
    let result = invalid_ptr::<u8>(addr);
}

