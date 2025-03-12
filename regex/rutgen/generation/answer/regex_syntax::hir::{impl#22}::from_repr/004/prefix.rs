// Answer 0

#[test]
fn test_from_repr_word_end_half_ascii() {
    let repr: u32 = 0b00_1000_0000_0000_0000; 
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_boundary_cases() {
    let repr_valid_start: u32 = 0b00_0000_0000_0000_0001; 
    let result_start = Look::from_repr(repr_valid_start);

    let repr_valid_end: u32 = 0b00_0000_0000_0000_0010; 
    let result_end = Look::from_repr(repr_valid_end);

    let repr_invalid: u32 = 0b11_1111_1111_1111_1111; 
    let result_invalid = Look::from_repr(repr_invalid);
}

