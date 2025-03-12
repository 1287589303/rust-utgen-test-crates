pub fn new(ch: char, spacing: Spacing) -> Self {
        if let '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';'
        | '<' | '=' | '>' | '?' | '@' | '^' | '|' | '~' = ch
        {
            Punct {
                ch,
                spacing,
                span: Span::call_site(),
            }
        } else {
            panic!("unsupported proc macro punctuation character {:?}", ch);
        }
    }