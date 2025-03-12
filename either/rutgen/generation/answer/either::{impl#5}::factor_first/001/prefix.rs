// Answer 0

#[test]
fn test_factor_first_with_numeric_left_and_string_right() {
    let input: Either<(u32, String), _> = Right((42, String::from("Hello")));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_with_numeric_left_and_vec_u8_right() {
    let input: Either<(i64, Vec<u8>), _> = Right((100, vec![1, 2, 3]));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_with_numeric_left_and_vec_string_right() {
    let input: Either<(usize, Vec<String>), _> = Right((7, vec![String::from("Test1"), String::from("Test2")]));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_with_float_left_and_string_right() {
    let input: Either<(f32, String), _> = Right((3.14, String::from("Pi")));
    let result = input.factor_first();
}

