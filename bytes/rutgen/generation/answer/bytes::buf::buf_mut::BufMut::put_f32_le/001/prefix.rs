// Answer 0

#[test]
fn test_put_f32_le_valid_value() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_f32_le(1.2f32);
}

#[test]
fn test_put_f32_le_min_value() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_f32_le(-3.40282347e+38f32);
}

#[test]
fn test_put_f32_le_max_value() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_f32_le(3.40282347e+38f32);
}

#[test]
fn test_put_f32_le_nan() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_f32_le(f32::NAN);
}

#[test]
fn test_put_f32_le_infinity() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_f32_le(f32::INFINITY);
}

#[test]
#[should_panic]
fn test_put_f32_le_insufficient_capacity() {
    let mut buf: Vec<u8> = vec![0; 0];
    buf.put_f32_le(1.0f32);
}

