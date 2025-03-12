// Answer 0

#[test]
fn test_div100() {
    assert_eq!(div100(0), 0);
    assert_eq!(div100(99), 0);
    assert_eq!(div100(100), 1);
    assert_eq!(div100(250), 2);
    assert_eq!(div100(500), 5);
    assert_eq!(div100(999), 9);
    assert_eq!(div100(10000), 100);
    assert_eq!(div100(u64::MAX), 184467440737095516).unwrap();
}

