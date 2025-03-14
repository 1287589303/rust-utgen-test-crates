// Answer 0

#[test]
fn test_new_with_zero_encoded_len() {
    struct Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl Decoder {
        pub(crate) fn new(encoded_len: usize) -> Self {
            let rem = encoded_len % 4;
            Self {
                rem,
                conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
            }
        }
    }

    let decoder = Decoder::new(0);
    assert_eq!(decoder.rem, 0);
    assert_eq!(decoder.conservative_decoded_len, 0);
}

#[test]
fn test_new_with_one_encoded_len() {
    struct Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl Decoder {
        pub(crate) fn new(encoded_len: usize) -> Self {
            let rem = encoded_len % 4;
            Self {
                rem,
                conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
            }
        }
    }

    let decoder = Decoder::new(1);
    assert_eq!(decoder.rem, 1);
    assert_eq!(decoder.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_two_encoded_len() {
    struct Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl Decoder {
        pub(crate) fn new(encoded_len: usize) -> Self {
            let rem = encoded_len % 4;
            Self {
                rem,
                conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
            }
        }
    }

    let decoder = Decoder::new(2);
    assert_eq!(decoder.rem, 2);
    assert_eq!(decoder.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_three_encoded_len() {
    struct Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl Decoder {
        pub(crate) fn new(encoded_len: usize) -> Self {
            let rem = encoded_len % 4;
            Self {
                rem,
                conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
            }
        }
    }

    let decoder = Decoder::new(3);
    assert_eq!(decoder.rem, 3);
    assert_eq!(decoder.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_four_encoded_len() {
    struct Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl Decoder {
        pub(crate) fn new(encoded_len: usize) -> Self {
            let rem = encoded_len % 4;
            Self {
                rem,
                conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
            }
        }
    }

    let decoder = Decoder::new(4);
    assert_eq!(decoder.rem, 0);
    assert_eq!(decoder.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_five_encoded_len() {
    struct Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl Decoder {
        pub(crate) fn new(encoded_len: usize) -> Self {
            let rem = encoded_len % 4;
            Self {
                rem,
                conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
            }
        }
    }

    let decoder = Decoder::new(5);
    assert_eq!(decoder.rem, 1);
    assert_eq!(decoder.conservative_decoded_len, 6);
}

