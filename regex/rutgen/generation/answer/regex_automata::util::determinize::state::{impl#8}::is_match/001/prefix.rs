// Answer 0

#[test]
fn test_is_match_with_zero() {
    let repr = Repr(&[0u8]);
    repr.is_match();
}

#[test]
fn test_is_match_with_one() {
    let repr = Repr(&[1u8]);
    repr.is_match();
}

#[test]
fn test_is_match_with_two() {
    let repr = Repr(&[2u8]);
    repr.is_match();
}

#[test]
fn test_is_match_with_ff() {
    let repr = Repr(&[255u8]);
    repr.is_match();
}

#[test]
fn test_is_match_with_boundary_case_high() {
    let repr = Repr(&[127u8]);
    repr.is_match();
}

#[test]
fn test_is_match_with_boundary_case_low() {
    let repr = Repr(&[0u8]);
    repr.is_match();
}

