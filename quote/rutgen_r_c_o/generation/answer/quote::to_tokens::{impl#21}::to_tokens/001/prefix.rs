// Answer 0

#[test]
fn test_to_tokens_zero() {
    let value: f64 = 0.0;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_negative_zero() {
    let value: f64 = -0.0;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_positive_infinity() {
    let value: f64 = std::f64::INFINITY;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_negative_infinity() {
    let value: f64 = std::f64::NEG_INFINITY;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_nan() {
    let value: f64 = std::f64::NAN;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_min_boundary() {
    let value: f64 = -1.0e+308;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_max_boundary() {
    let value: f64 = 1.0e+308;
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

