// Answer 0

#[test]
fn test_to_tokens() {
    use quote::TokenStream;
    use quote::Literal;

    struct MyStruct(i8);

    impl MyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i8_suffixed(self.0));
        }
    }

    let mut tokens = TokenStream::new();
    let my_struct = MyStruct(42);
    my_struct.to_tokens(&mut tokens);

    assert_eq!(tokens.to_string(), "42i8");
}

