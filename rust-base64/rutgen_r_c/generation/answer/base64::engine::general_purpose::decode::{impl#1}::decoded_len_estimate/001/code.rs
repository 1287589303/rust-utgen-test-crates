// Answer 0

#[test]
fn test_decoded_len_estimate_zero() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = TestEstimate {
        rem: 0,
        conservative_decoded_len: 0,
    };
    assert_eq!(estimate.decoded_len_estimate(), 0);
}

#[test]
fn test_decoded_len_estimate_positive() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = TestEstimate {
        rem: 1,
        conservative_decoded_len: 10,
    };
    assert_eq!(estimate.decoded_len_estimate(), 10);
}

#[test]
fn test_decoded_len_estimate_large_value() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = TestEstimate {
        rem: 2,
        conservative_decoded_len: usize::MAX,
    };
    assert_eq!(estimate.decoded_len_estimate(), usize::MAX);
}

