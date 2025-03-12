// Answer 0

#[test]
fn test_contains_range_empty_set() {
    let set = ByteSet::empty();
    assert!(set.contains_range(0, 0));
    assert!(set.contains_range(0, 255));
    assert!(set.contains_range(200, 250));
    assert!(set.contains_range(255, 255));
}

#[test]
fn test_contains_range_single_element() {
    let mut set = ByteSet::empty();
    set.add(5);
    assert!(set.contains_range(5, 5));
    assert!(!set.contains_range(0, 0));
    assert!(!set.contains_range(10, 10));
}

#[test]
fn test_contains_range_all_elements() {
    let mut set = ByteSet::empty();
    for byte in 0..=255 {
        set.add(byte);
    }
    assert!(set.contains_range(0, 255));
}

#[test]
fn test_contains_range_overlapping_range() {
    let mut set = ByteSet::empty();
    for byte in 250..=255 {
        set.add(byte);
    }
    assert!(set.contains_range(250, 255));
    assert!(!set.contains_range(0, 10));
}

#[test]
fn test_contains_range_equal_values() {
    let mut set = ByteSet::empty();
    set.add(100);
    assert!(set.contains_range(100, 100));
    assert!(!set.contains_range(99, 99));
}

#[test]
fn test_contains_range_boundary_cases() {
    let mut set = ByteSet::empty();
    set.add(0);
    set.add(255);
    assert!(set.contains_range(0, 0));
    assert!(set.contains_range(255, 255));
    assert!(!set.contains_range(0, 255));
}

