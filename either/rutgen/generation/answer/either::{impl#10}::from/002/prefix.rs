// Answer 0

#[test]
fn test_from_result_with_string() {
    let result: Result<String, &str> = Ok(String::from("Test String"));
    let either: Either<&str, String> = Either::from(result);
}

#[test]
fn test_from_result_with_integer() {
    let result: Result<i32, &str> = Ok(42);
    let either: Either<&str, i32> = Either::from(result);
}

#[test]
fn test_from_result_with_float() {
    let result: Result<f64, &str> = Ok(3.14);
    let either: Either<&str, f64> = Either::from(result);
}

#[test]
fn test_from_result_with_vector() {
    let result: Result<Vec<i32>, &str> = Ok(vec![1, 2, 3]);
    let either: Either<&str, Vec<i32>> = Either::from(result);
}

#[test]
fn test_from_result_with_tuple() {
    let result: Result<(i32, i32), &str> = Ok((1, 2));
    let either: Either<&str, (i32, i32)> = Either::from(result);
}

