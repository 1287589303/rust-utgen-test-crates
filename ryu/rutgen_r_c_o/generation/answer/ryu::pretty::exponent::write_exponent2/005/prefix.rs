// Answer 0

#[test]
fn test_write_exponent2_zero() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 0;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_one() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 1;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_two() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 2;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_three() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 3;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_four() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 4;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_five() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 5;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_six() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 6;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_seven() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 7;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_eight() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 8;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

#[test]
fn test_write_exponent2_nine() {
    let mut result = [0u8; 3];
    let result_ptr = result.as_mut_ptr();
    let k: isize = 9;
    unsafe {
        let return_value = write_exponent2(k, result_ptr);
    }
}

