pub(crate) fn from_str_checked(repr: &str) -> Result<Self, LexError> {
        let mut cursor = get_cursor(repr);
        #[cfg(span_locations)]
        let lo = cursor.off;

        let negative = cursor.starts_with_char('-');
        if negative {
            cursor = cursor.advance(1);
            if !cursor.starts_with_fn(|ch| ch.is_ascii_digit()) {
                return Err(LexError::call_site());
            }
        }

        if let Ok((rest, mut literal)) = parse::literal(cursor) {
            if rest.is_empty() {
                if negative {
                    literal.repr.insert(0, '-');
                }
                literal.span = Span {
                    #[cfg(span_locations)]
                    lo,
                    #[cfg(span_locations)]
                    hi: rest.off,
                };
                return Ok(literal);
            }
        }
        Err(LexError::call_site())
    }