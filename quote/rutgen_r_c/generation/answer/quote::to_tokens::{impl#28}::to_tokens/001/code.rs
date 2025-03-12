// Answer 0

#[test]
fn test_to_tokens_with_punct() {
    use proc_macro2::{TokenStream, Punct};

    let mut token_stream = TokenStream::new();
    let punct = Punct::new('!', proc_macro2::Spacing::Alone);
    
    punct.to_tokens(&mut token_stream);
    
    let expected: TokenStream = TokenStream::from(punct.clone());
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_multiple_puncts() {
    use proc_macro2::{TokenStream, Punct};

    let mut token_stream = TokenStream::new();

    let punct1 = Punct::new('!', proc_macro2::Spacing::Alone);
    let punct2 = Punct::new('?', proc_macro2::Spacing::Alone);
    
    punct1.to_tokens(&mut token_stream);
    punct2.to_tokens(&mut token_stream);

    let expected: TokenStream = TokenStream::from(vec![punct1.clone(), punct2.clone()]);
    assert_eq!(token_stream.to_string(), expected.to_string());
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_to_tokens_with_punct_should_panic() {
    use proc_macro2::{TokenStream, Punct};

    let mut token_stream = TokenStream::new();
    let punct = Punct::new('!', proc_macro2::Spacing::Alone);

    punct.to_tokens(&mut token_stream);
    
    let wrong_expected: TokenStream = TokenStream::from(Punct::new('?', proc_macro2::Spacing::Alone));
    assert_eq!(token_stream.to_string(), wrong_expected.to_string());
}

