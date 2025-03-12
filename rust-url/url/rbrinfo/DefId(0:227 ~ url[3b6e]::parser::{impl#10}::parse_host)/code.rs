pub fn parse_host(
        mut input: Input<'_>,
        scheme_type: SchemeType,
    ) -> ParseResult<(Host<String>, Input<'_>)> {
        if scheme_type.is_file() {
            return Parser::get_file_host(input);
        }
        // Undo the Input abstraction here to avoid allocating in the common case
        // where the host part of the input does not contain any tab or newline
        let input_str = input.chars.as_str();
        let mut inside_square_brackets = false;
        let mut has_ignored_chars = false;
        let mut non_ignored_chars = 0;
        let mut bytes = 0;
        for c in input_str.chars() {
            match c {
                ':' if !inside_square_brackets => break,
                '\\' if scheme_type.is_special() => break,
                '/' | '?' | '#' => break,
                ascii_tab_or_new_line_pattern!() => {
                    has_ignored_chars = true;
                }
                '[' => {
                    inside_square_brackets = true;
                    non_ignored_chars += 1
                }
                ']' => {
                    inside_square_brackets = false;
                    non_ignored_chars += 1
                }
                _ => non_ignored_chars += 1,
            }
            bytes += c.len_utf8();
        }
        let replaced: String;
        let host_str;
        {
            let host_input = input.by_ref().take(non_ignored_chars);
            if has_ignored_chars {
                replaced = host_input.collect();
                host_str = &*replaced
            } else {
                for _ in host_input {}
                host_str = &input_str[..bytes]
            }
        }
        if scheme_type == SchemeType::SpecialNotFile && host_str.is_empty() {
            return Err(ParseError::EmptyHost);
        }
        if !scheme_type.is_special() {
            let host = Host::parse_opaque(host_str)?;
            return Ok((host, input));
        }
        let host = Host::parse(host_str)?;
        Ok((host, input))
    }