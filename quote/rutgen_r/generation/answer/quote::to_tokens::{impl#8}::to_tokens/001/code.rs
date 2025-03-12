// Answer 0

#[derive(Debug)]
struct MyStruct {
    value: i8,
}

impl MyStruct {
    fn new(value: i8) -> Self {
        MyStruct { value }
    }
}

#[test]
fn test_to_tokens_with_positive_value() {
    let mut tokens = quote::TokenStream::new();
    let my_struct = MyStruct::new(5);
    my_struct.to_tokens(&mut tokens);
    let expected = quote::quote! { 5_i8 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_negative_value() {
    let mut tokens = quote::TokenStream::new();
    let my_struct = MyStruct::new(-3);
    my_struct.to_tokens(&mut tokens);
    let expected = quote::quote! { -3_i8 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[test]
fn test_to_tokens_with_zero_value() {
    let mut tokens = quote::TokenStream::new();
    let my_struct = MyStruct::new(0);
    my_struct.to_tokens(&mut tokens);
    let expected = quote::quote! { 0_i8 };
    assert_eq!(tokens.to_string(), expected.to_string());
}

#[should_panic]
fn test_to_tokens_with_overflow_value() {
    let mut tokens = quote::TokenStream::new();
    let my_struct = MyStruct::new(128); // Oversized for i8
    my_struct.to_tokens(&mut tokens);
}

