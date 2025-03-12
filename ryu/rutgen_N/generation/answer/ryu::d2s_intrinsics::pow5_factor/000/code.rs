// Answer 0

#[test]
fn test_pow5_factor() {
    assert_eq!(pow5_factor(1), 0);
    assert_eq!(pow5_factor(5), 1);
    assert_eq!(pow5_factor(25), 2);
    assert_eq!(pow5_factor(125), 3);
    assert_eq!(pow5_factor(625), 4);
    assert_eq!(pow5_factor(3125), 5);
    assert_eq!(pow5_factor(15625), 6);
    assert_eq!(pow5_factor(78125), 7);
    assert_eq!(pow5_factor(390625), 8);
    assert_eq!(pow5_factor(1953125), 9);
    assert_eq!(pow5_factor(9765625), 10);
    assert_eq!(pow5_factor(48828125), 11);
    assert_eq!(pow5_factor(u64::MAX), 0); // Testing the upper limit
}

