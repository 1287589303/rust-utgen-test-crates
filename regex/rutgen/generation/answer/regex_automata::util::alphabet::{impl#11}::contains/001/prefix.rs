// Answer 0

#[test]
fn test_contains_empty_byte_set() {
    let byte_set = ByteSet::empty();
    assert!(!byte_set.contains(0));
    assert!(!byte_set.contains(127));
    assert!(!byte_set.contains(128));
    assert!(!byte_set.contains(255));
}

#[test]
fn test_contains_single_byte() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(0);
    assert!(byte_set.contains(0));
    assert!(!byte_set.contains(1));

    byte_set.add(255);
    assert!(byte_set.contains(255));
    assert!(!byte_set.contains(254));
}

#[test]
fn test_contains_range_bytes() {
    let mut byte_set = ByteSet::empty();
    for byte in 0..=255 {
        byte_set.add(byte);
    }
    for byte in 0..=255 {
        assert!(byte_set.contains(byte));
    }
}

#[test]
fn test_contains_boundaries() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(127);
    byte_set.add(128);
    
    assert!(byte_set.contains(127));
    assert!(byte_set.contains(128));
    assert!(!byte_set.contains(126));
    assert!(!byte_set.contains(129));
}

#[test]
fn test_contains_non_contained_bytes() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(1);
    byte_set.add(2);
    byte_set.add(3);
    
    assert!(byte_set.contains(1));
    assert!(byte_set.contains(2));
    assert!(byte_set.contains(3));
    assert!(!byte_set.contains(0));
    assert!(!byte_set.contains(4));
}

