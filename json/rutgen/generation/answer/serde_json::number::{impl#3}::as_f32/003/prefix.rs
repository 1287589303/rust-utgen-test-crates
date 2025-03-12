// Answer 0

#[test]
fn test_as_f32_with_pos_int_0() {
    let number = Number { n: N::PosInt(0) };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_with_pos_int_1() {
    let number = Number { n: N::PosInt(1) };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_with_pos_int_16777215() {
    let number = Number { n: N::PosInt(16777215) };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_with_pos_int_2() {
    let number = Number { n: N::PosInt(2) };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_with_pos_int_16384() {
    let number = Number { n: N::PosInt(16384) };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_with_pos_int_123456() {
    let number = Number { n: N::PosInt(123456) };
    let _ = number.as_f32();
}

