// Answer 0

#[test]
fn test_check_alignment_fail_case1() {
    let slice: &[u8] = &[1, 2, 3]; // Address will not be aligned for u8
    let result: Result<(), DeserializeError> = check_alignment::<u32>(slice);
}

#[test]
fn test_check_alignment_fail_case2() {
    let slice: &[u8] = &[0; 3]; // Address will not be aligned for u64
    let result: Result<(), DeserializeError> = check_alignment::<u64>(slice);
}

#[test]
fn test_check_alignment_fail_case3() {
    let slice: &[u8] = &[0; 1]; // Address will not be aligned for u16
    let result: Result<(), DeserializeError> = check_alignment::<u16>(slice);
}

#[test]
fn test_check_alignment_fail_case4() {
    let slice: &[u8] = &[0; 5]; // Address will not be aligned for u8
    let result: Result<(), DeserializeError> = check_alignment::<u64>(slice);
}

#[test]
fn test_check_alignment_fail_case5() {
    let slice: &[u8] = &[0; 7]; // Address will not be aligned for u8
    let result: Result<(), DeserializeError> = check_alignment::<u32>(slice);
}

