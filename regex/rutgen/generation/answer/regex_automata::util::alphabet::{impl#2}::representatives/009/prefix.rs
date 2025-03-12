// Answer 0

#[test]
fn test_representatives_excluded_bounds_1() {
    let mut byte_classes = ByteClasses::empty();
    let reps: ByteClassRepresentatives = byte_classes.representatives(b'\x01'..b'\x03');
}

#[test]
fn test_representatives_excluded_bounds_2() {
    let mut byte_classes = ByteClasses::singletons();
    let reps: ByteClassRepresentatives = byte_classes.representatives(b'\x02'..b'\x05');
}

#[test]
fn test_representatives_excluded_bounds_3() {
    let mut byte_classes = ByteClasses::empty();
    let reps: ByteClassRepresentatives = byte_classes.representatives(b'\x10'..b'\x12');
}

#[test]
fn test_representatives_excluded_bounds_boundary() {
    let mut byte_classes = ByteClasses::singletons();
    let reps: ByteClassRepresentatives = byte_classes.representatives(b'\x0F'..b'\x10');
}

