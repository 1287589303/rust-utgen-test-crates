// Answer 0

#[test]
fn test_div5() {
    assert_eq!(div5(0), 0);
    assert_eq!(div5(5), 1);
    assert_eq!(div5(10), 2);
    assert_eq!(div5(12), 2);
    assert_eq!(div5(20), 4);
    assert_eq!(div5(25), 5);
    assert_eq!(div5(30), 6);
    assert_eq!(div5(100), 20);
    assert_eq!(div5(1_000_000), 200_000);
}

