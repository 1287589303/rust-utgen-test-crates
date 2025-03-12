// Answer 0

#[test]
fn test_to_tokens_with_string_literal() {
    struct MyString<'a>(&'a str);

    impl ToTokens for MyString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = MyString("hello");
    let mut output = TokenStream::new();
    input.to_tokens(&mut output);
    let expected_output: TokenStream = Literal::string("hello").into();

    assert_eq!(output.to_string(), expected_output.to_string());
}

#[test]
fn test_to_tokens_with_empty_string() {
    struct MyString<'a>(&'a str);

    impl ToTokens for MyString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = MyString("");
    let mut output = TokenStream::new();
    input.to_tokens(&mut output);
    let expected_output: TokenStream = Literal::string("").into();

    assert_eq!(output.to_string(), expected_output.to_string());
}

#[test]
fn test_to_tokens_with_special_characters() {
    struct MyString<'a>(&'a str);

    impl ToTokens for MyString<'_> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::string(self.0));
        }
    }

    let input = MyString("special!@#");
    let mut output = TokenStream::new();
    input.to_tokens(&mut output);
    let expected_output: TokenStream = Literal::string("special!@#").into();

    assert_eq!(output.to_string(), expected_output.to_string());
}

