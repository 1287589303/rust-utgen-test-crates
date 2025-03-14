// Answer 0

#[derive(Debug)]
struct GeneralPurposeEstimate {
    length: usize,
}

impl GeneralPurposeEstimate {
    fn new(length: usize) -> Self {
        GeneralPurposeEstimate { length }
    }
}

trait Decoder {
    type DecodeEstimate;

    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate;
}

struct SimpleDecoder;

impl Decoder for SimpleDecoder {
    type DecodeEstimate = GeneralPurposeEstimate;

    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        GeneralPurposeEstimate::new(input_len)
    }
}

#[test]
fn test_internal_decoded_len_estimate_zero() {
    let decoder = SimpleDecoder;
    let estimate = decoder.internal_decoded_len_estimate(0);
    assert_eq!(estimate.length, 0);
}

#[test]
fn test_internal_decoded_len_estimate_positive() {
    let decoder = SimpleDecoder;
    let estimate = decoder.internal_decoded_len_estimate(10);
    assert_eq!(estimate.length, 10);
}

#[test]
fn test_internal_decoded_len_estimate_large_number() {
    let decoder = SimpleDecoder;
    let estimate = decoder.internal_decoded_len_estimate(usize::MAX);
    assert_eq!(estimate.length, usize::MAX);
}

