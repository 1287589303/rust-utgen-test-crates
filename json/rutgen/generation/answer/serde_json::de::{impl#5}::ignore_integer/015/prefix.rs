// Answer 0

#[test]
fn test_ignore_integer_zero_followed_by_invalid() {
    let input = vec![b'0', b'1']; // '0' followed by '1' should be invalid
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_zero_followed_by_zero() {
    let input = vec![b'0', b'0']; // only leading '0' is allowed
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_char() {
    let input = vec![b'3', b'@']; // valid integer '3' followed by invalid character '@'
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_integer_followed_by_decimal() {
    let input = vec![b'4', b'.']; // valid integer '4' followed by decimal separator
    let mut deserializer = Deserializer {
        read: SliceRead::new(&input),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

