// Answer 0

#[test]
fn test_mul_shift_all_64() {
    // Test case setup
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let m: u64 = 5;
    let mul: (u64, u64) = (3, 2);
    let j: u32 = 64;
    let mm_shift: u32 = 1;

    unsafe {
        // Call the function under test
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);

        // Verifying expected results
        assert_eq!(result, mul_shift_64(4 * m, &mul, j));
        assert_eq!(vp, mul_shift_64(4 * m + 2, &mul, j));
        assert_eq!(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
    }
}

#[test]
fn test_mul_shift_all_64_with_zero() {
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let m: u64 = 0;
    let mul: (u64, u64) = (0, 0);
    let j: u32 = 64;
    let mm_shift: u32 = 0;

    unsafe {
        // Call the function under test
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);

        // Verifying expected results
        assert_eq!(result, mul_shift_64(4 * m, &mul, j));
        assert_eq!(vp, mul_shift_64(4 * m + 2, &mul, j));
        assert_eq!(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
    }
}

#[test]
fn test_mul_shift_all_64_edge_case() {
    let mut vp: u64 = 0;
    let mut vm: u64 = 0;
    let m: u64 = u64::MAX;
    let mul: (u64, u64) = (u64::MAX, u64::MAX);
    let j: u32 = 64;
    let mm_shift: u32 = 0;

    unsafe {
        // Call the function under test
        let result = mul_shift_all_64(m, &mul, j, &mut vp as *mut u64, &mut vm as *mut u64, mm_shift);

        // Verifying expected results
        assert_eq!(result, mul_shift_64(4 * m, &mul, j));
        assert_eq!(vp, mul_shift_64(4 * m + 2, &mul, j));
        assert_eq!(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, &mul, j));
    }
}

