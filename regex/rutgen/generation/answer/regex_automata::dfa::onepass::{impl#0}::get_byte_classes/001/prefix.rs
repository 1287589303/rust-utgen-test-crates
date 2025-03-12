// Answer 0

#[test]
fn test_get_byte_classes_true() {
    let config = Config::new().byte_classes(true);
    let _ = config.get_byte_classes();
}

#[test]
fn test_get_byte_classes_false() {
    let config = Config::new().byte_classes(false);
    let _ = config.get_byte_classes();
}

#[test]
fn test_get_byte_classes_none() {
    let config = Config::new().byte_classes(None);
    let _ = config.get_byte_classes();
}

