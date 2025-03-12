// Answer 0

#[test]
fn test_representatives_included_bounds() {
    let mut byte_classes = ByteClasses::empty();
    byte_classes.set(b'a', 1);
    byte_classes.set(b'b', 2);
    let reps = byte_classes.representatives(0..=1);
}

#[test]
fn test_representatives_included_bounds_different_classes() {
    let mut byte_classes = ByteClasses::empty();
    byte_classes.set(b'0', 1);
    byte_classes.set(b'1', 2);
    byte_classes.set(b'2', 3);
    let reps = byte_classes.representatives(0..=2);
}

#[test]
fn test_representatives_included_bounds_edge_case() {
    let mut byte_classes = ByteClasses::empty();
    byte_classes.set(b'\x01', 1);
    byte_classes.set(b'\xFF', 2);
    let reps = byte_classes.representatives(1..=255);
}

