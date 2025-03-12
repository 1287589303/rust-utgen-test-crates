// Answer 0

#[test]
fn test_stride2_empty() {
    let byte_classes = ByteClasses::empty();
    let _ = byte_classes.stride2();
}

#[test]
fn test_stride2_singletons() {
    let byte_classes = ByteClasses::singletons();
    let _ = byte_classes.stride2();
}

#[test]
fn test_stride2_full_alphabet() {
    let mut byte_classes = ByteClasses::empty();
    for byte in 0..256 {
        byte_classes.set(byte, 1);
    }
    let _ = byte_classes.stride2();
}

#[test]
fn test_stride2_some_bytes_set() {
    let mut byte_classes = ByteClasses::empty();
    byte_classes.set(0, 1);
    byte_classes.set(1, 1);
    byte_classes.set(2, 1);
    let _ = byte_classes.stride2();
}

#[test]
fn test_stride2_no_bytes_set() {
    let mut byte_classes = ByteClasses::empty();
    let _ = byte_classes.stride2();
}

