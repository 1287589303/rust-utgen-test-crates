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
fn test_new_with_encoded_len_multiple_of_four() {
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

    let decoder = Decoder::new(8);
    assert_eq!(decoder.rem, 0);
    assert_eq!(decoder.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_encoded_len_one_more_than_multiple_of_four() {
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
    assert_eq!(decoder.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_encoded_len_two_more_than_multiple_of_four() {
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

    let decoder = Decoder::new(6);
    assert_eq!(decoder.rem, 2);
    assert_eq!(decoder.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_encoded_len_three_more_than_multiple_of_four() {
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

    let decoder = Decoder::new(7);
    assert_eq!(decoder.rem, 3);
    assert_eq!(decoder.conservative_decoded_len, 6);
}

