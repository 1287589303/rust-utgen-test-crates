// Answer 0

#[test]
fn test_new_start_byte_map_custom_lineterm() {
    struct TestLookMatcher {
        lineterm: DebugByte,
    }

    impl TestLookMatcher {
        fn new(lineterm: u8) -> Self {
            TestLookMatcher {
                lineterm: DebugByte(lineterm),
            }
        }

        fn get_line_terminator(&self) -> u8 {
            self.lineterm.0
        }
    }

    let lookm = TestLookMatcher::new(0xFF); // Custom line terminator greater than b'\r' and not equal to b'\n'
    let start_byte_map = StartByteMap::new(&lookm);
}

#[test]
fn test_new_start_byte_map_custom_lineterm_2() {
    struct TestLookMatcher {
        lineterm: DebugByte,
    }

    impl TestLookMatcher {
        fn new(lineterm: u8) -> Self {
            TestLookMatcher {
                lineterm: DebugByte(lineterm),
            }
        }

        fn get_line_terminator(&self) -> u8 {
            self.lineterm.0
        }
    }

    let lookm = TestLookMatcher::new(0xFE); // Another custom line terminator
    let start_byte_map = StartByteMap::new(&lookm);
}

#[test]
fn test_new_start_byte_map_custom_lineterm_3() {
    struct TestLookMatcher {
        lineterm: DebugByte,
    }

    impl TestLookMatcher {
        fn new(lineterm: u8) -> Self {
            TestLookMatcher {
                lineterm: DebugByte(lineterm),
            }
        }

        fn get_line_terminator(&self) -> u8 {
            self.lineterm.0
        }
    }

    let lookm = TestLookMatcher::new(0xA0); // Yet another custom line terminator
    let start_byte_map = StartByteMap::new(&lookm);
}

