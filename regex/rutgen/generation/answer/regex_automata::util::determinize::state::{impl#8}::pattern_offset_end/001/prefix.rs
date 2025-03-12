// Answer 0

#[test]
fn test_pattern_offset_end_case_1() {
    let pattern_data = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]; // Encodes length 1
    let repr = Repr(pattern_data);
    let _ = repr.pattern_offset_end();
}

#[test]
fn test_pattern_offset_end_case_2() {
    let pattern_data = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0]; // Encodes length 2
    let repr = Repr(pattern_data);
    let _ = repr.pattern_offset_end();
}

#[test]
fn test_pattern_offset_end_case_3() {
    let pattern_data = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0]; // Encodes length 3
    let repr = Repr(pattern_data);
    let _ = repr.pattern_offset_end();
}

#[test]
fn test_pattern_offset_end_case_4() {
    let pattern_data = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0]; // Encodes length 4
    let repr = Repr(pattern_data);
    let _ = repr.pattern_offset_end();
}

