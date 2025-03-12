pub(crate) fn token_stream(mut input: Cursor) -> Result<TokenStream, LexError> {
    let mut trees = TokenStreamBuilder::new();
    let mut stack = Vec::new();

    loop {
        input = skip_whitespace(input);

        if let Ok((rest, ())) = doc_comment(input, &mut trees) {
            input = rest;
            continue;
        }

        #[cfg(span_locations)]
        let lo = input.off;

        let first = match input.bytes().next() {
            Some(first) => first,
            None => match stack.last() {
                None => return Ok(trees.build()),
                #[cfg(span_locations)]
                Some((lo, _frame)) => {
                    return Err(LexError {
                        span: Span { lo: *lo, hi: *lo },
                    })
                }
                #[cfg(not(span_locations))]
                Some(_frame) => return Err(LexError { span: Span {} }),
            },
        };

        if let Some(open_delimiter) = match first {
            b'(' if !input.starts_with(ERROR) => Some(Delimiter::Parenthesis),
            b'[' => Some(Delimiter::Bracket),
            b'{' => Some(Delimiter::Brace),
            _ => None,
        } {
            input = input.advance(1);
            let frame = (open_delimiter, trees);
            #[cfg(span_locations)]
            let frame = (lo, frame);
            stack.push(frame);
            trees = TokenStreamBuilder::new();
        } else if let Some(close_delimiter) = match first {
            b')' => Some(Delimiter::Parenthesis),
            b']' => Some(Delimiter::Bracket),
            b'}' => Some(Delimiter::Brace),
            _ => None,
        } {
            let frame = match stack.pop() {
                Some(frame) => frame,
                None => return Err(lex_error(input)),
            };
            #[cfg(span_locations)]
            let (lo, frame) = frame;
            let (open_delimiter, outer) = frame;
            if open_delimiter != close_delimiter {
                return Err(lex_error(input));
            }
            input = input.advance(1);
            let mut g = Group::new(open_delimiter, trees.build());
            g.set_span(Span {
                #[cfg(span_locations)]
                lo,
                #[cfg(span_locations)]
                hi: input.off,
            });
            trees = outer;
            trees.push_token_from_parser(TokenTree::Group(crate::Group::_new_fallback(g)));
        } else {
            let (rest, mut tt) = match leaf_token(input) {
                Ok((rest, tt)) => (rest, tt),
                Err(Reject) => return Err(lex_error(input)),
            };
            tt.set_span(crate::Span::_new_fallback(Span {
                #[cfg(span_locations)]
                lo,
                #[cfg(span_locations)]
                hi: rest.off,
            }));
            trees.push_token_from_parser(tt);
            input = rest;
        }
    }
}