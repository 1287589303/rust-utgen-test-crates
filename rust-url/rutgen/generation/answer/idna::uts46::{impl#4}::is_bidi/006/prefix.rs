// Answer 0

#[test]
fn test_is_bidi_with_specific_characters() {
    struct TestWrite;
    impl core::fmt::Write for TestWrite {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let uts46 = Uts46::new();
    
    let buffer: &[char] = &['\u{0590}', '\u{10A00}']; // '\u{10A00}' is in range ['\u{11000}', '\u{1E7FF}']
    let result = uts46.is_bidi(buffer);
}

#[test]
fn test_is_bidi_no_bidi_chars() {
    struct TestWrite;
    impl core::fmt::Write for TestWrite {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let uts46 = Uts46::new();
    
    let buffer: &[char] = &['a', 'b', 'c']; // All characters below '\u{0590}'
    let result = uts46.is_bidi(buffer);
}

