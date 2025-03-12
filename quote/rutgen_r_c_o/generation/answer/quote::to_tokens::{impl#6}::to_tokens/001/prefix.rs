// Answer 0

#[test]
fn test_to_tokens_non_empty_string() {
    struct TestString<'a>(&'a str);

    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = TestString("Hello, World!");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_empty_string() {
    struct TestString<'a>(&'a str);

    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = TestString("");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_special_characters() {
    struct TestString<'a>(&'a str);

    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = TestString("Special !@#$%^&*()_+?");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_non_ascii_characters() {
    struct TestString<'a>(&'a str);

    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = TestString("こんにちは世界");
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_max_length_string() {
    struct TestString<'a>(&'a str);

    impl ToTokens for TestString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = TestString("a".repeat(1_000_000)); // Example of a long string.
    let mut tokens = TokenStream::new();
    input.to_tokens(&mut tokens);
}

