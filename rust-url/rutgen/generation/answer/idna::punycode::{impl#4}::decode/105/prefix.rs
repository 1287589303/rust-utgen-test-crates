// Answer 0

#[test]
fn test_decode_with_error_on_exceeding_biased_condition() {
    use core::char;

    struct TestCodeUnit(char);

    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { self.0 == '-' }
        fn is_ascii(&self) -> bool { self.0.is_ascii() }
        fn digit(&self) -> Option<u32> {
            if self.0.is_ascii_digit() {
                self.0.to_digit(10)
            } else {
                None
            }
        }
        fn char(&self) -> char { self.0 }
        fn char_ascii_lower_case(&self) -> char { self.0.to_ascii_lowercase() }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let mut decoder = Decoder::default();
    let input: Vec<TestCodeUnit> = vec![
        TestCodeUnit('é'), 
        TestCodeUnit('ç'), 
        TestCodeUnit('-'), 
        TestCodeUnit('1'), 
        TestCodeUnit('2'), 
        TestCodeUnit('3')
    ];
    let _ = decoder.decode::<TestCodeUnit, TestCaller>(&input);
}

