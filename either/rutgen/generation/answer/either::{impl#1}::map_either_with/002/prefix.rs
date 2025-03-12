// Answer 0

#[test]
fn test_map_either_with_left_variant() {
    let mut sum = 0;
    let f = |ctx: &mut usize, s: String| {
        *ctx += s.len();
        s.to_uppercase()
    };
    let left: Either<String, usize> = Left("test".into());
    let result = left.map_either_with(&mut sum, f, |_, u| u.to_string());
}

#[test]
fn test_map_either_with_left_variant_empty_string() {
    let mut sum = 0;
    let f = |ctx: &mut usize, s: String| {
        *ctx += s.len();
        s.to_uppercase()
    };
    let left: Either<String, usize> = Left("".into());
    let result = left.map_either_with(&mut sum, f, |_, u| u.to_string());
}

#[test]
fn test_map_either_with_left_variant_long_string() {
    let mut sum = 0;
    let f = |ctx: &mut usize, s: String| {
        *ctx += s.len();
        s.to_uppercase()
    };
    let left: Either<String, usize> = Left("longer test string".into());
    let result = left.map_either_with(&mut sum, f, |_, u| u.to_string());
}

