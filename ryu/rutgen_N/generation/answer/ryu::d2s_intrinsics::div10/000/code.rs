// Answer 0

#[test]
fn test_div10() {
    assert_eq!(div10(0), 0);
    assert_eq!(div10(10), 1);
    assert_eq!(div10(20), 2);
    assert_eq!(div10(99), 9);
    assert_eq!(div10(100), 10);
    assert_eq!(div10(1000), 100);
    assert_eq!(div10(10000), 1000);
    assert_eq!(div10(u64::MAX), 1844674407370955161); // 2^64 - 1 divided by 10
}

