// Answer 0

#[test]
fn test_encode_into_non_ascii_with_overflow() {
    struct TestCaller {
        phantom: PhantomData<()>,
    }

    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input: Vec<char> = vec!['é', 'ä']; // Non-ASCII input
    let input_iter = input.iter().cloned();
    let mut output = String::new();

    let _ = encode_into::<_, _, TestCaller>(input_iter, &mut output);
}

#[test]
fn test_encode_into_non_ascii_with_error() {
    struct TestCaller {
        phantom: PhantomData<()>,
    }

    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = true;
    }

    let input: Vec<char> = vec!['ç', '汉']; // Non-ASCII input
    let input_iter = input.iter().cloned();
    let mut output = String::new();

    let _ = encode_into::<_, _, TestCaller>(input_iter, &mut output);
}

