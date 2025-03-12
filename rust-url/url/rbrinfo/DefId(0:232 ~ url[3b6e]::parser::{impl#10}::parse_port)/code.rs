pub fn parse_port<P>(
        mut input: Input<'_>,
        default_port: P,
        context: Context,
    ) -> ParseResult<(Option<u16>, Input<'_>)>
    where
        P: Fn() -> Option<u16>,
    {
        let mut port: u32 = 0;
        let mut has_any_digit = false;
        while let (Some(c), remaining) = input.split_first() {
            if let Some(digit) = c.to_digit(10) {
                port = port * 10 + digit;
                if port > u16::MAX as u32 {
                    return Err(ParseError::InvalidPort);
                }
                has_any_digit = true;
            } else if context == Context::UrlParser && !matches!(c, '/' | '\\' | '?' | '#') {
                return Err(ParseError::InvalidPort);
            } else {
                break;
            }
            input = remaining;
        }

        if !has_any_digit && context == Context::Setter && !input.is_empty() {
            return Err(ParseError::InvalidPort);
        }

        let mut opt_port = Some(port as u16);
        if !has_any_digit || opt_port == default_port() {
            opt_port = None;
        }
        Ok((opt_port, input))
    }