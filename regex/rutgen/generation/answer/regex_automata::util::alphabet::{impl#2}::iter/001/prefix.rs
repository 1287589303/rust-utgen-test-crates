// Answer 0

#[test]
fn test_iter_empty() {
    let byte_classes = ByteClasses::empty();
    let iter = byte_classes.iter();
}

#[test]
fn test_iter_singletons() {
    let byte_classes = ByteClasses::singletons();
    let iter = byte_classes.iter();
}

#[test]
fn test_iter_from_bytes_empty_slice() {
    let byte_classes = ByteClasses::from_bytes(&[]).unwrap().0;
    let iter = byte_classes.iter();
}

#[test]
fn test_iter_from_bytes_full_slice() {
    let byte_classes = ByteClasses::from_bytes(&[0; 256]).unwrap().0;
    let iter = byte_classes.iter();
}

