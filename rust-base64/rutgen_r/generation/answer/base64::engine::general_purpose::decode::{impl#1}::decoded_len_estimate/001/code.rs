// Answer 0

#[derive(Debug)]
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
fn test_decoded_len_estimate_zero() {
    let decoder = Decoder::new(0);
    assert_eq!(decoder.decoded_len_estimate(), 0);
}

#[test]
fn test_decoded_len_estimate_small() {
    let decoder = Decoder::new(10);
    assert_eq!(decoder.decoded_len_estimate(), 10);
}

#[test]
fn test_decoded_len_estimate_large() {
    let decoder = Decoder::new(1000);
    assert_eq!(decoder.decoded_len_estimate(), 1000);
}

#[test]
fn test_decoded_len_estimate_boundary() {
    let decoder = Decoder::new(usize::MAX);
    assert_eq!(decoder.decoded_len_estimate(), usize::MAX);
}

