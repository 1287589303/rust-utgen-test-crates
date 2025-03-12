// Answer 0

#[test]
fn test_ignore_integer_valid_single_digit() {
    let input = vec![b'1', b'2', b'3', b'e', b'0'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_multiple_digits() {
    let input = vec![b'1', b'2', b'3', b'4', b'5', b'e', b'0'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_leading_zero() {
    let input = vec![b'0', b'1'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid_zero() {
    let input = vec![b'0', b'.', b'5'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_characters() {
    let input = vec![b'A'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid_exponent() {
    let input = vec![b'3', b'e', b'5'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_decimal() {
    let input = vec![b'7', b'.', b'2', b'5'];
    let mut deserializer = Deserializer { read: MockRead::new(&input), scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.ignore_integer();
} 

struct MockRead<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> MockRead<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    fn next_char(&mut self) -> Result<Option<u8>> {
        if self.position < self.data.len() {
            let ch = self.data[self.position];
            self.position += 1;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }

    fn peek(&self) -> Result<Option<u8>> {
        if self.position < self.data.len() {
            Ok(Some(self.data[self.position]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.position += 1;
    }
}

