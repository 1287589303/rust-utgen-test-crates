// Answer 0

#[test]
fn test_div100() {
    assert_eq!(div100(100), 1);
    assert_eq!(div100(200), 2);
    assert_eq!(div100(99), 0);
    assert_eq!(div100(0), 0);
    assert_eq!(div100(250), 2);
    assert_eq!(div100(10000), 100);
}

#[test]
fn test_div100_large_numbers() {
    assert_eq!(div100(1_000_000), 10_000);
    assert_eq!(div100(10_000_000), 100_000);
    assert_eq!(div100(u64::MAX), 18_446_744_073_709_551); // Checking max u64 value
}

