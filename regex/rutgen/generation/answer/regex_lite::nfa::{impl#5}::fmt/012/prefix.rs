// Answer 0

#[test]
fn test_state_split_empty_targets_not_reversed() {
    use core::fmt::Formatter;

    struct TestFormatter;
    
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let targets: Vec<u32> = vec![];
    let reverse = false;
    let state = State::Splits { targets: targets.clone(), reverse };
    
    let mut formatter = TestFormatter;
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_state_split_empty_targets_reversed() {
    use core::fmt::Formatter;

    struct TestFormatter;
    
    impl core::fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let targets: Vec<u32> = vec![];
    let reverse = true;
    let state = State::Splits { targets: targets.clone(), reverse };
    
    let mut formatter = TestFormatter;
    let _ = state.fmt(&mut formatter);
}

