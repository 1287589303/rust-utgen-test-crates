// Answer 0

#[test]
fn test_to_tokens_positive_float() {
    use proc_macro2::{TokenStream, Literal};
    use super::ToTokens;

    struct FloatWrapper(f32);

    let mut tokens = TokenStream::new();
    let float_value = FloatWrapper(1.23);

    float_value.to_tokens(&mut tokens);
    let expected = Literal::f32_suffixed(1.23).to_string();

    assert!(tokens.to_string().contains(&expected));
}

#[test]
fn test_to_tokens_zero_float() {
    use proc_macro2::{TokenStream, Literal};
    use super::ToTokens;

    struct FloatWrapper(f32);

    let mut tokens = TokenStream::new();
    let float_value = FloatWrapper(0.0);

    float_value.to_tokens(&mut tokens);
    let expected = Literal::f32_suffixed(0.0).to_string();

    assert!(tokens.to_string().contains(&expected));
}

#[test]
fn test_to_tokens_negative_float() {
    use proc_macro2::{TokenStream, Literal};
    use super::ToTokens;

    struct FloatWrapper(f32);

    let mut tokens = TokenStream::new();
    let float_value = FloatWrapper(-1.23);

    float_value.to_tokens(&mut tokens);
    let expected = Literal::f32_suffixed(-1.23).to_string();

    assert!(tokens.to_string().contains(&expected));
}

