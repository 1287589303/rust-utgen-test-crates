// Answer 0

#[test]
fn test_map_either_left_non_empty_string() {
    let f = |s: String| s.len();
    let left: Either<String, u8> = Left("hello".into());
    let _ = left.map_either(f, |x| x.to_string());
}

#[test]
fn test_map_either_left_empty_string() {
    let f = |s: String| s.len();
    let left: Either<String, u8> = Left("".into());
    let _ = left.map_either(f, |x| x.to_string());
}

#[test]
fn test_map_either_left_string_with_zero_length() {
    let f = |s: String| s.len();
    let left: Either<String, u8> = Left("".into());
    let _ = left.map_either(f, |x| x.to_string());
}

#[test]
fn test_map_either_left_with_u8_zero() {
    let f = |s: String| s.len();
    let left: Either<String, u8> = Left("zero".into());
    let _ = left.map_either(f, |x| x.to_string()); 
}

#[test]
fn test_map_either_left_with_u8_max() {
    let f = |s: String| s.len();
    let left: Either<String, u8> = Left("maximum".into());
    let _ = left.map_either(f, |x| x.to_string()); 
}

