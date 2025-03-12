pub fn parse_opaque(input: &str) -> Result<Self, ParseError> {
        if input.starts_with('[') {
            if !input.ends_with(']') {
                return Err(ParseError::InvalidIpv6Address);
            }
            return parse_ipv6addr(&input[1..input.len() - 1]).map(Host::Ipv6);
        }

        let is_invalid_host_char = |c| {
            matches!(
                c,
                '\0' | '\t'
                    | '\n'
                    | '\r'
                    | ' '
                    | '#'
                    | '/'
                    | ':'
                    | '<'
                    | '>'
                    | '?'
                    | '@'
                    | '['
                    | '\\'
                    | ']'
                    | '^'
                    | '|'
            )
        };

        if input.find(is_invalid_host_char).is_some() {
            Err(ParseError::InvalidDomainCharacter)
        } else {
            Ok(Host::Domain(
                utf8_percent_encode(input, CONTROLS).to_string(),
            ))
        }
    }