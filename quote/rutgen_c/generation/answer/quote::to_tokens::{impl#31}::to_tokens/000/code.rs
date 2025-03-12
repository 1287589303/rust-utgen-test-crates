// Answer 0

#[test]
fn test_to_tokens() {
    use proc_macro2::TokenStream;
    
    let ts1 = TokenStream::new();
    let mut ts2 = TokenStream::new();
    
    ts1.to_tokens(&mut ts2);
    
    assert_eq!(ts2.to_string(), ts1.to_string());
}

#[test]
fn test_to_token_stream() {
    use proc_macro2::TokenStream;

    let ts1 = TokenStream::new();
    let ts2 = ts1.to_token_stream();

    assert_eq!(ts2.to_string(), ts1.to_string());
}

#[test]
fn test_into_token_stream() {
    use proc_macro2::TokenStream;

    let ts1 = TokenStream::new();
    let ts2 = ts1.clone().into_token_stream();

    assert_eq!(ts2.to_string(), ts1.to_string());
}

