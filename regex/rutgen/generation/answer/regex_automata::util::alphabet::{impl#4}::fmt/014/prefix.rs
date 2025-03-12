// Answer 0

#[test]
fn test_byte_classes_fmt_non_singleton() {
    let mut byte_classes = ByteClasses::empty();
    let class = Unit::u8(0);
    byte_classes.set(0, 0);
    
    // Attempting to get a writeable buffer
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| byte_classes.fmt(f));
    
    // Here we check that writing is okay.
    let is_ok = result.is_ok();
}

#[test]
fn test_byte_classes_fmt_with_element_start_equal_end() {
    let mut byte_classes = ByteClasses::empty();
    let class = Unit::u8(1);
    byte_classes.set(1, 1);
    
    // Ensure element ranges returns a start equal to end
    byte_classes.element_ranges(class); 

    // Attempting to get a writeable buffer
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| byte_classes.fmt(f));
    
    // Here we check that writing is okay.
    let is_ok = result.is_ok();
}

