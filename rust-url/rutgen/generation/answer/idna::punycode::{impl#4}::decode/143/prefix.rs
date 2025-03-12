// Answer 0

#[test]
fn test_decode_with_non_ascii_characters_and_delimiter_at_start() {
    struct NonAsciiCodeUnit {
        value: char,
    }

    impl PunycodeCodeUnit for NonAsciiCodeUnit {
        fn is_delimiter(&self) -> bool {
            self.value == '-'
        }

        fn is_ascii(&self) -> bool {
            self.value.is_ascii()
        }

        fn digit(&self) -> Option<u32> {
            None
        }

        fn char(&self) -> char {
            self.value
        }

        fn char_ascii_lower_case(&self) -> char {
            self.value.to_ascii_lowercase()
        }
    }

    struct DummyCaller;
    impl PunycodeCaller for DummyCaller {
        const EXTERNAL_CALLER: bool = true;
    }

    let mut decoder = Decoder::default();
    let input = [
        NonAsciiCodeUnit { value: '-' },
        NonAsciiCodeUnit { value: '日' },
        NonAsciiCodeUnit { value: '本' },
    ];
    
    let _result = decoder.decode::<NonAsciiCodeUnit, DummyCaller>(&input);
}

#[test]
fn test_decode_with_special_characters_and_delimiter_at_start() {
    struct SpecialCodeUnit {
        value: char,
    }

    impl PunycodeCodeUnit for SpecialCodeUnit {
        fn is_delimiter(&self) -> bool {
            self.value == '@'
        }

        fn is_ascii(&self) -> bool {
            self.value.is_ascii()
        }

        fn digit(&self) -> Option<u32> {
            None
        }

        fn char(&self) -> char {
            self.value
        }

        fn char_ascii_lower_case(&self) -> char {
            self.value.to_ascii_lowercase()
        }
    }

    struct DummyCaller;
    impl PunycodeCaller for DummyCaller {
        const EXTERNAL_CALLER: bool = true;
    }

    let mut decoder = Decoder::default();
    let input = [
        SpecialCodeUnit { value: '@' },
        SpecialCodeUnit { value: 'Ω' },
        SpecialCodeUnit { value: '∞' },
    ];
    
    let _result = decoder.decode::<SpecialCodeUnit, DummyCaller>(&input);
}

