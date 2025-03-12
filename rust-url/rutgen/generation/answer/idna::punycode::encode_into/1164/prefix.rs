// Answer 0

#[test]
fn test_encode_into_non_ascii_characters() {
    struct NonAsciiCaller;
    impl PunycodeCaller for NonAsciiCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = vec!['é', '汉', '😀'];
    let mut output = String::new();
    let result = encode_into(input.iter().cloned(), &mut output);
    let _ = result.unwrap();
}

#[test]
fn test_encode_into_single_non_ascii_character() {
    struct NonAsciiCaller;
    impl PunycodeCaller for NonAsciiCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = vec!['汉'];
    let mut output = String::new();
    let result = encode_into(input.iter().cloned(), &mut output);
    let _ = result.unwrap();
}

#[test]
fn test_encode_into_multiple_non_ascii_characters() {
    struct NonAsciiCaller;
    impl PunycodeCaller for NonAsciiCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = vec!['é', 'é', '漢', '✈', '🐍'];
    let mut output = String::new();
    let result = encode_into(input.iter().cloned(), &mut output);
    let _ = result.unwrap();
}

