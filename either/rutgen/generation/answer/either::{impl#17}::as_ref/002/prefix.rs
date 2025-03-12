// Answer 0

#[test]
fn test_either_as_ref_left_empty_slice() {
    let left: Either<&[u8], &str> = Either::Left(&[]);
    let _result: &[u8] = left.as_ref();
}

#[test]
fn test_either_as_ref_left_non_empty_slice() {
    let left: Either<&[u8], &str> = Either::Left(&[1, 2, 3]);
    let _result: &[u8] = left.as_ref();
}

#[test]
fn test_either_as_ref_right_string() {
    let right: Either<&[u8], &str> = Either::Right("Hello");
    let _result: &[u8] = right.as_ref();
}

#[test]
fn test_either_as_ref_right_char_slice() {
    let right: Either<&[u8], &[char]> = Either::Right(&['a', 'b', 'c']);
    let _result: &[u8] = right.as_ref();
}

