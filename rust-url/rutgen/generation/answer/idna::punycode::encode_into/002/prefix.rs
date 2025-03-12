// Answer 0

#[test]
fn test_encode_into_with_valid_ascii() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = "abc".chars().collect();
    let mut output: String = String::new();

    let result = encode_into::<_, _, TestCaller>(input.iter().copied(), &mut output);
}

#[test]
fn test_encode_into_with_exceeding_basic_ascii() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = "a".repeat(2048).chars().collect();
    let mut output: String = String::new();

    let result = encode_into::<_, _, TestCaller>(input.iter().copied(), &mut output);
}

#[test]
fn test_encode_into_with_single_character() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = "x".chars().collect();
    let mut output: String = String::new();

    let result = encode_into::<_, _, TestCaller>(input.iter().copied(), &mut output);
}

