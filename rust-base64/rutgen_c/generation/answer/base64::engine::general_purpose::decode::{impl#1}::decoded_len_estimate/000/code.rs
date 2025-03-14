// Answer 0

#[test]
fn test_decoded_len_estimate() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = GeneralPurposeEstimate {
        rem: 2,
        conservative_decoded_len: 10,
    };

    assert_eq!(estimate.decoded_len_estimate(), 10);
}

#[test]
fn test_decoded_len_estimate_zero() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 0,
    };

    assert_eq!(estimate.decoded_len_estimate(), 0);
} 

#[test]
fn test_decoded_len_estimate_large_value() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = GeneralPurposeEstimate {
        rem: 1,
        conservative_decoded_len: usize::MAX,
    };

    assert_eq!(estimate.decoded_len_estimate(), usize::MAX);
}

