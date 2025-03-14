// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoded_len_estimate() {
        assert_eq!(3, decoded_len_estimate(1));
        assert_eq!(3, decoded_len_estimate(2));
        assert_eq!(3, decoded_len_estimate(3));
        assert_eq!(3, decoded_len_estimate(4));
        assert_eq!(6, decoded_len_estimate(5));
    }
}

