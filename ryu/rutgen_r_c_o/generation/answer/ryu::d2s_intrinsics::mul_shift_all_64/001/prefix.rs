// Answer 0

#[test]
fn test_mul_shift_all_64_case_0() {
    let m: u64 = 0;
    let mul: (u64, u64) = (0, 0);
    let j: u32 = 64;
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 0;
    unsafe {
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case_max_m() {
    let m: u64 = u64::MAX;
    let mul: (u64, u64) = (u64::MAX, u64::MAX);
    let j: u32 = 128;
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 63;
    unsafe {
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case_boundary_j() {
    let m: u64 = 1;
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 64;
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 0;
    unsafe {
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

#[test]
fn test_mul_shift_all_64_case_max_j() {
    let m: u64 = 2;
    let mul: (u64, u64) = (2, 2);
    let j: u32 = 128;
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 32;
    unsafe {
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
} 

#[test]
fn test_mul_shift_all_64_case_random_valid() {
    let m: u64 = 123456789;
    let mul: (u64, u64) = (987654321, 123456789);
    let j: u32 = 100;
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let mm_shift: u32 = 15;
    unsafe {
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);
    }
}

