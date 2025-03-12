// Answer 0

#[test]
fn test_into_token_stream() {
    struct TokenStream;

    impl TokenStream {
        fn new() -> Self {
            TokenStream
        }
    }

    let ts = TokenStream::new();
    assert_eq!(ts.into_token_stream(), ts);
}

