// Answer 0

#[test]
fn test_unexpected_map() {
    use std::fmt;
    use crate::lib::*;
    
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    
    let mut formatter = TestFormatter;
    let unexpected_map = Unexpected::Map;
    let _ = unexpected_map.fmt(&mut formatter);
}

#[test]
fn test_unexpected_map_display() {
    use std::fmt;
    use crate::lib::*;
    
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    
    let mut formatter = TestFormatter;
    let unexpected_map = Unexpected::Map;
    let _ = write!(formatter, "{}", unexpected_map);
}

