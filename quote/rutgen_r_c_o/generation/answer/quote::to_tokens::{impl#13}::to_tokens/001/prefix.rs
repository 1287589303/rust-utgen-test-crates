// Answer 0

#[test]
fn test_to_tokens_positive_integer() {
    struct PositiveInteger(i32);
    impl ToTokens for PositiveInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }
    let value = PositiveInteger(42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_negative_integer() {
    struct NegativeInteger(i32);
    impl ToTokens for NegativeInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }
    let value = NegativeInteger(-42);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_zero() {
    struct ZeroInteger(i32);
    impl ToTokens for ZeroInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }
    let value = ZeroInteger(0);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_min_integer() {
    struct MinInteger(i32);
    impl ToTokens for MinInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }
    let value = MinInteger(i32::MIN);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

#[test]
fn test_to_tokens_max_integer() {
    struct MaxInteger(i32);
    impl ToTokens for MaxInteger {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::isize_suffixed(self.0));
        }
    }
    let value = MaxInteger(i32::MAX);
    let mut tokens = TokenStream::new();
    value.to_tokens(&mut tokens);
}

