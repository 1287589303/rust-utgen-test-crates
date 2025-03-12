// Answer 0

#[test]
fn test_join_spans_with_multiple_tokens() {
    struct Token {
        span: Span,
    }

    impl Token {
        fn span(&self) -> Span {
            self.span
        }
    }

    let tokens = vec![
        Token { span: Span::from(1..2) },
        Token { span: Span::from(2..3) },
        Token { span: Span::from(3..4) },
    ];
    
    let token_stream = TokenStream::from_iter(tokens.into_iter());
    
    let result = join_spans(token_stream);
    assert_eq!(result, Span::from(1..4));
}

#[test]
fn test_join_spans_with_no_tokens() {
    let tokens: Vec<Token> = vec![];
    let token_stream = TokenStream::from_iter(tokens.into_iter());
    
    let result = join_spans(token_stream);
    assert_eq!(result, Span::call_site());
}

#[test]
fn test_join_spans_with_single_token() {
    struct Token {
        span: Span,
    }
    
    impl Token {
        fn span(&self) -> Span {
            self.span
        }
    }

    let token = Token { span: Span::from(1..2) };
    let token_stream = TokenStream::from_iter(vec![token].into_iter());
    
    let result = join_spans(token_stream);
    assert_eq!(result, Span::from(1..2));
}

