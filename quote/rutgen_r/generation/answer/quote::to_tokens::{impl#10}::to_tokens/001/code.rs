// Answer 0

#[test]
fn test_to_tokens_with_positive_i32() {
    struct MyStruct(i32);

    impl MyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(*self));
        }
    }

    let mut tokens = TokenStream::new();
    let input = MyStruct(42);
    input.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "42i32");
}

#[test]
fn test_to_tokens_with_zero() {
    struct MyStruct(i32);

    impl MyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(*self));
        }
    }

    let mut tokens = TokenStream::new();
    let input = MyStruct(0);
    input.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "0i32");
}

#[test]
fn test_to_tokens_with_negative_i32() {
    struct MyStruct(i32);

    impl MyStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::i32_suffixed(*self));
        }
    }

    let mut tokens = TokenStream::new();
    let input = MyStruct(-42);
    input.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), "-42i32");
}

