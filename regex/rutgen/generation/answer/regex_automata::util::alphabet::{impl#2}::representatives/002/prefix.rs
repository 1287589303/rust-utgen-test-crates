// Answer 0

#[test]
fn test_representatives_unbounded_range() {
    let byte_classes = ByteClasses::empty();
    let representatives = byte_classes.representatives(..);
}

#[test]
fn test_representatives_left_unbounded() {
    let byte_classes = ByteClasses::singletons();
    let representatives = byte_classes.representatives(..);
}

#[test]
fn test_representatives_right_unbounded() {
    let byte_classes = ByteClasses::empty();
    let representatives = byte_classes.representatives(..);
}

