// Answer 0

#[derive(Default)]
struct Decoder {
    conservative_decoded_len: usize,
}

impl Decoder {
    fn new(conservative_decoded_len: usize) -> Self {
        Decoder { conservative_decoded_len }
    }

    fn decoded_len_estimate(&self) -> usize {
        self.conservative_decoded_len
    }
}

#[test]
fn test_decoded_len_estimate() {
    let decoder = Decoder::new(42);
    assert_eq!(decoder.decoded_len_estimate(), 42);
}

#[test]
fn test_decoded_len_estimate_zero() {
    let decoder = Decoder::new(0);
    assert_eq!(decoder.decoded_len_estimate(), 0);
}

