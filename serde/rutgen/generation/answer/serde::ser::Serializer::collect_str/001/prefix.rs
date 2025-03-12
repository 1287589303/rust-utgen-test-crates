// Answer 0

#[test]
fn test_collect_str_empty_string() {
    struct Empty;

    impl std::fmt::Display for Empty {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "")
        }
    }

    let serializer = T;
    let _ = serializer.collect_str(&Empty);
}

#[test]
fn test_collect_str_special_characters() {
    struct SpecialChars;

    impl std::fmt::Display for SpecialChars {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "!@#$%^&*()_+")
        }
    }

    let serializer = T;
    let _ = serializer.collect_str(&SpecialChars);
}

#[test]
fn test_collect_str_long_string() {
    struct LongString;

    impl std::fmt::Display for LongString {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a".repeat(1000).as_str())
        }
    }

    let serializer = T;
    let _ = serializer.collect_str(&LongString);
}

#[test]
fn test_collect_str_non_ascii_characters() {
    struct NonAscii;

    impl std::fmt::Display for NonAscii {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "你好，世界")
        }
    }

    let serializer = T;
    let _ = serializer.collect_str(&NonAscii);
}

