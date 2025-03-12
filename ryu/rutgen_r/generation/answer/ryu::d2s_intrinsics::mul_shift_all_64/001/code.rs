// Answer 0

#[test]
fn test_mul_shift_all_64() {
    use std::ptr;

    let m: u64 = 5;
    let mul: (u64, u64) = (2, 3);
    let j: u32 = 1;
    
    let mut vp_value: u64 = 0;
    let mut vm_value: u64 = 0;
    let vp: *mut u64 = &mut vp_value;
    let vm: *mut u64 = &mut vm_value;
    let mm_shift: u32 = 2;

    unsafe {
        let result = mul_shift_all_64(m, &mul, j, vp, vm, mm_shift);
        
        assert_eq!(vp_value, mul_shift_64(4 * m + 2, &mul, j));
        assert_eq!(vm_value, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
        assert_eq!(result, mul_shift_64(4 * m, &mul, j));
    }
}

fn mul_shift_64(value: u64, mul: &(u64, u64), j: u32) -> u64 {
    // This is a mock implementation for the purpose of the test
    let (m1, m2) = *mul;
    value.wrapping_mul(m1).wrapping_add(m2).wrapping_add(j as u64)
}

#[test]
fn test_mul_shift_all_64_zero_m() {
    use std::ptr;

    let m: u64 = 0;
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 0;
    
    let mut vp_value: u64 = 0;
    let mut vm_value: u64 = 0;
    let vp: *mut u64 = &mut vp_value;
    let vm: *mut u64 = &mut vm_value;
    let mm_shift: u32 = 0;

    unsafe {
        let result = mul_shift_all_64(m, &mul, j, vp, vm, mm_shift);
        
        assert_eq!(vp_value, mul_shift_64(4 * m + 2, &mul, j));
        assert_eq!(vm_value, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
        assert_eq!(result, mul_shift_64(4 * m, &mul, j));
    }
}

#[test]
#[should_panic]
fn test_mul_shift_all_64_invalid_pointer() {
    let m: u64 = 1;
    let mul: (u64, u64) = (1, 2);
    let j: u32 = 1;
    
    let vp: *mut u64 = std::ptr::null_mut();
    let vm: *mut u64 = std::ptr::null_mut();
    let mm_shift: u32 = 1;

    unsafe {
        mul_shift_all_64(m, &mul, j, vp, vm, mm_shift);
    }
}

