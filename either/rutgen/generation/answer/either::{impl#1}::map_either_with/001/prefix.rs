// Answer 0

#[test]
fn test_map_either_with_right_variant_integer() {
    let mut sum = 0;
    let f = |_: &mut usize, _: String| 0; // No impact on sum for left
    let g = |sum: &mut usize, u: usize| { *sum += u; u.to_string() };
    
    let right: Either<String, usize> = Right(42);
    let result = right.map_either_with(&mut sum, &f, &g);
}

#[test]
fn test_map_either_with_right_variant_string() {
    let mut sum = 0;
    let f = |_: &mut usize, _: String| 0; // No impact on sum for left
    let g = |sum: &mut usize, u: usize| { *sum += u; u.to_string() };

    let right: Either<String, usize> = Right(58);
    let result = right.map_either_with(&mut sum, &f, &g);
}

#[test]
fn test_map_either_with_right_variant_floating_point() {
    let mut sum = 0;
    let f = |_: &mut usize, _: String| 0; // No impact on sum for left
    let g = |sum: &mut usize, u: f64| { *sum += u as usize; u.to_string() };

    let right: Either<String, f64> = Right(3.14);
    let result = right.map_either_with(&mut sum, &f, &g);
}

