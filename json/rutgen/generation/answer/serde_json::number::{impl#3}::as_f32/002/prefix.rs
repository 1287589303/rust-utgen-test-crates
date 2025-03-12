// Answer 0

#[test]
fn test_as_f32_neg_int_boundary_case_1() {
    let number = Number { n: N::NegInt(-1) };
    let _result = number.as_f32();
}

#[test]
fn test_as_f32_neg_int_boundary_case_2() {
    let number = Number { n: N::NegInt(-2147483648) };
    let _result = number.as_f32();
}

#[test]
fn test_as_f32_neg_int_case() {
    let number = Number { n: N::NegInt(-1000) };
    let _result = number.as_f32();
}

#[test]
fn test_as_f32_neg_int_case_max() {
    let number = Number { n: N::NegInt(-100) };
    let _result = number.as_f32();
}

