// Answer 0

#[test]
fn test_count_digits_in_range_10000_to_65535() {
    let inputs = [10000, 20000, 32768, 45000, 65535];
    for &n in &inputs {
        let _result = count_digits(n);
    }
}

