// Answer 0

#[test]
fn test_factor_first_left() {
    let input: Either<(i32, f64), (i32, String)> = Left((42, 3.14));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_right() {
    let input: Either<(i32, Vec<u8>), (i32, String)> = Right((42, String::from("test")));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_left_strings() {
    let input: Either<(String, bool), (String, Vec<u8>)> = Left((String::from("hello"), true));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_right_floats() {
    let input: Either<(f64, Vec<i32>), (f64, String)> = Right((3.14, String::from("world")));
    let result = input.factor_first();
}

#[test]
fn test_factor_first_empty_vector() {
    let input: Either<(usize, Vec<u8>), (usize, String)> = Left((0, Vec::new()));
    let result = input.factor_first();
}

