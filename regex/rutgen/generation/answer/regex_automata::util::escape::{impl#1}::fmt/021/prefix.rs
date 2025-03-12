// Answer 0

#[test]
fn test_debug_haystack_with_control_characters_and_bounds() {
    struct TestStruct<'a>(&'a [u8]);

    let input = [
        0x0b, // \x0b (vertical tab)
        0x0c, // \x0c (form feed)
        0x0e, // \x0e (control character)
        0x7f, // \x7f (delete)
        0x01, // \x01 (start of heading)
        0x02, // \x02 (start of text)
        0x03, // \x03 (end of text)
        0x04, // \x04 (end of transmission)
        0x05, // \x05 (enquiry)
        0x06, // \x06 (acknowledge)
        0x07, // \x07 (bell)
        0x08, // \x08 (backspace)
        0x0a, // \n (line feed)
        0x0d, // \r (carriage return)
        0x09, // \t (tab)
        0x0f, // \x0f (control character)
    ];
    
    let debug_haystack = TestStruct(&input);
    let _ = core::fmt::Debug::fmt(&debug_haystack, &mut core::fmt::Formatter::new());
}

#[test]
fn test_debug_haystack_with_exclusive_ranges() {
    struct TestStruct<'a>(&'a [u8]);

    let input = [
        0x0e, // \x0e
        0x12, // \x12
        0x19, // \x19
        0x7f, // \x7f
        0x0b, // \x0b
        0x0c, // \x0c
        0x08, // \x08
    ];
    
    let debug_haystack = TestStruct(&input);
    let _ = core::fmt::Debug::fmt(&debug_haystack, &mut core::fmt::Formatter::new());
}

