// Answer 0

#[test]
fn test_left_or_else_with_string_left() {
    let left: Either<String, u32> = Left("hello".to_string());
    let result = left.left_or_else(|_| "unreachable".to_string());
}

#[test]
fn test_left_or_else_with_string_right() {
    let right: Either<String, u32> = Right(42);
    let result = right.left_or_else(|x| (x * 2).to_string());
}

#[test]
fn test_left_or_else_with_vec_left() {
    let left: Either<Vec<u8>, i32> = Left(vec![1, 2, 3]);
    let result = left.left_or_else(|_| vec![0]);
}

#[test]
fn test_left_or_else_with_vec_right() {
    let right: Either<Vec<u8>, i32> = Right(5);
    let result = right.left_or_else(|x| vec![x]);
}

#[test]
fn test_left_or_else_with_tuple_left() {
    let left: Either<(i32, i32), f32> = Left((1, 2));
    let result = left.left_or_else(|_| (0, 0));
}

#[test]
fn test_left_or_else_with_tuple_right() {
    let right: Either<(i32, i32), f32> = Right(3.14);
    let result = right.left_or_else(|x| (x as i32, x as i32));
}

#[test]
fn test_left_or_else_with_custom_struct_left() {
    struct Custom {
        value: i32,
    }
    
    let left: Either<Custom, f64> = Left(Custom { value: 10 });
    let result = left.left_or_else(|_| Custom { value: 0 });
}

#[test]
fn test_left_or_else_with_custom_struct_right() {
    struct Custom {
        value: i32,
    }
    
    let right: Either<Custom, f64> = Right(4.5);
    let result = right.left_or_else(|x| Custom { value: x as i32 });
}

