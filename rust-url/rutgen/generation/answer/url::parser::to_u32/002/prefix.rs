// Answer 0

#[test]
fn test_to_u32_overflow_case_1() {
    let input: usize = u32::MAX as usize + 1;
    let result = to_u32(input);
}

#[test]
fn test_to_u32_overflow_case_2() {
    let input: usize = std::usize::MAX;
    let result = to_u32(input);
}

#[test]
fn test_to_u32_overflow_case_3() {
    let input: usize = u32::MAX as usize + 100;
    let result = to_u32(input);
}

