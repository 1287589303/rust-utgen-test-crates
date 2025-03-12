// Answer 0

#[test]
fn test_left_and_then_with_non_empty_left() {
    let left: Either<i32, &str> = Left(5);
    let result = left.left_and_then(|x| Right(x * 2));
}

#[test]
fn test_left_and_then_with_non_empty_left_failing() {
    let left: Either<String, &str> = Left(String::from("test"));
    let result = left.left_and_then(|x| Right(x.len() as isize));
}

#[test]
fn test_left_and_then_with_complex_type() {
    let left: Either<(i32, i32), f64> = Left((1, 2));
    let result = left.left_and_then(|(x, y)| Right(x + y as f64));
}

#[test]
fn test_left_and_then_with_array() {
    let left: Either<[i32; 1], f64> = Left([42]);
    let result = left.left_and_then(|arr| Right(arr[0] as f64));
}

