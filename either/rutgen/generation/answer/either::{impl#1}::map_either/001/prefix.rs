// Answer 0

#[test]
fn test_map_either_with_right_variant() {
    let g = |u: u8| u.to_string();

    let right: Either<String, u8> = Right(0);
    let _ = right.map_either(|_s| 0, g);

    let right: Either<String, u8> = Right(128);
    let _ = right.map_either(|_s| 0, g);

    let right: Either<String, u8> = Right(255);
    let _ = right.map_either(|_s| 0, g);
}

