// Answer 0

#[test]
fn test_decode_with_empty_base_and_matching_digit() {
    struct TestCodeUnit {
        value: u32,
    }

    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool {
            self.value == 0x2D // ASCII for '-'
        }

        fn is_ascii(&self) -> bool {
            self.value < 128
        }

        fn digit(&self) -> Option<u32> {
            Some(self.value) // Assume value corresponds to a valid digit
        }

        fn char(&self) -> char {
            char::from_u32(self.value).unwrap()
        }

        fn char_ascii_lower_case(&self) -> char {
            self.char().to_ascii_lowercase()
        }
    }

    struct TestCaller;

    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let mut decoder = Decoder::default();

    let input: Vec<TestCodeUnit> = vec![
        TestCodeUnit { value: 0x2D }, // delimiter '-'
        TestCodeUnit { value: 0x61 }, // 'a'
        TestCodeUnit { value: 36 },    // valid digit (1)
    ];

    let result = decoder.decode::<TestCodeUnit, TestCaller>(&input);
}

