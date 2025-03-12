// Answer 0

#[test]
fn test_is_singleton_with_single_byte_and_eoi() {
    let mut byte_classes = ByteClasses([0; 256]);
    byte_classes.set(0, 1); // Setting byte 0 to its own class (1)
    byte_classes.set(255, 1); // Setting EOI byte to the same class (1)
    assert_eq!(byte_classes.is_singleton(), true);
}

#[test]
fn test_is_singleton_with_two_bytes_and_eoi() {
    let mut byte_classes = ByteClasses([0; 256]);
    byte_classes.set(0, 1); // Singleton for byte 0
    byte_classes.set(1, 2); // Singleton for byte 1
    byte_classes.set(255, 1); // EOI byte
    assert_eq!(byte_classes.is_singleton(), false);
}

#[test]
fn test_is_singleton_with_multiple_singletons() {
    let mut byte_classes = ByteClasses([0; 256]);
    for i in 0..255 {
        byte_classes.set(i, (i % 2) + 1); // Setting alternating bytes
    }
    byte_classes.set(255, 1); // EOI byte
    assert_eq!(byte_classes.is_singleton(), false);
}

#[test]
fn test_is_singleton_with_257_classes() {
    let mut byte_classes = ByteClasses([0; 256]);
    byte_classes.set(0, 1); // Singleton for byte 0
    byte_classes.set(255, 0); // EOI byte with a different class
    for i in 1..255 {
        byte_classes.set(i, i + 1); // Each subsequent byte gets its own class
    }
    assert_eq!(byte_classes.is_singleton(), false);
}

#[test]
fn test_is_singleton_all_singletons() {
    let mut byte_classes = ByteClasses([0; 256]);
    for i in 0..256 {
        byte_classes.set(i as u8, 1); // All bytes still map to the singleton
    }
    byte_classes.set(255, 1); // EOI byte
    assert_eq!(byte_classes.is_singleton(), true);
}

