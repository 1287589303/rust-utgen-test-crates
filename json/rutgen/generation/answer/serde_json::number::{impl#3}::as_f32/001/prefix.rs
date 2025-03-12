// Answer 0

#[test]
fn test_as_f32_positive_float() {
    let number = Number {
        n: N::Float(1.0),
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_negative_float() {
    let number = Number {
        n: N::Float(-1.0),
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_large_positive_float() {
    let number = Number {
        n: N::Float(f32::MAX as f64), 
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_large_negative_float() {
    let number = Number {
        n: N::Float(-f32::MAX as f64), 
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_small_positive_float() {
    let number = Number {
        n: N::Float(f32::MIN as f64), 
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_small_negative_float() {
    let number = Number {
        n: N::Float(-(f32::MIN as f64)), 
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_positive_infinity() {
    let number = Number {
        n: N::Float(f64::INFINITY), 
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_negative_infinity() {
    let number = Number {
        n: N::Float(f64::NEG_INFINITY), 
    };
    let _ = number.as_f32();
}

#[test]
fn test_as_f32_nan() {
    let number = Number {
        n: N::Float(f64::NAN), 
    };
    let _ = number.as_f32();
}

