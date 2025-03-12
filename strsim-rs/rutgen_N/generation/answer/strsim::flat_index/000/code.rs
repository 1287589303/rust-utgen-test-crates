// Answer 0

#[test]
fn test_flat_index() {
    assert_eq!(flat_index(0, 0, 5), 0);
    assert_eq!(flat_index(1, 0, 5), 1);
    assert_eq!(flat_index(0, 1, 5), 5);
    assert_eq!(flat_index(1, 1, 5), 6);
    assert_eq!(flat_index(4, 1, 5), 9);
    assert_eq!(flat_index(0, 2, 5), 10);
    assert_eq!(flat_index(4, 3, 5), 19);
}

