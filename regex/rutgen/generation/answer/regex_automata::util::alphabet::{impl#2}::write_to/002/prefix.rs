// Answer 0

#[test]
fn test_write_to_success() {
    let byte_classes = ByteClasses::empty();
    let mut dst = [0u8; 256];
    let result = byte_classes.write_to(&mut dst);
}

#[test]
fn test_write_to_single_byte() {
    let byte_classes = ByteClasses::singletons();
    let mut dst = [0u8; 256];
    let _ = byte_classes.write_to(&mut dst);
    for b in 0..=255 {
        assert_eq!(dst[b as usize], byte_classes.get(b));
    }
}

#[test]
#[should_panic]
fn test_write_to_too_small_buffer() {
    let byte_classes = ByteClasses::empty();
    let mut dst = [0u8; 255];
    let _ = byte_classes.write_to(&mut dst);
}

