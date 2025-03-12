fn parse_parameters(s: &str, parameters: &mut Vec<(String, String)>) {
    let mut semicolon_separated = s.split(';');

    while let Some(piece) = semicolon_separated.next() {
        let piece = piece.trim_start_matches(http_whitespace);
        let (name, value) = split2(piece, '=');
        // We can not early return on an invalid name here, because the value
        // parsing later may consume more semicolon seperated pieces.
        let name_valid =
            !name.is_empty() && only_http_token_code_points(name) && !contains(parameters, name);
        if let Some(value) = value {
            let value = if let Some(stripped) = value.strip_prefix('"') {
                let max_len = stripped.len().saturating_sub(1); // without end quote
                let mut unescaped_value = String::with_capacity(max_len);
                let mut chars = stripped.chars();
                'until_closing_quote: loop {
                    while let Some(c) = chars.next() {
                        match c {
                            '"' => break 'until_closing_quote,
                            '\\' => unescaped_value.push(chars.next().unwrap_or_else(|| {
                                semicolon_separated
                                    .next()
                                    .map(|piece| {
                                        // A semicolon inside a quoted value is not a separator
                                        // for the next parameter, but part of the value.
                                        chars = piece.chars();
                                        ';'
                                    })
                                    .unwrap_or('\\')
                            })),
                            _ => unescaped_value.push(c),
                        }
                    }
                    if let Some(piece) = semicolon_separated.next() {
                        // A semicolon inside a quoted value is not a separator
                        // for the next parameter, but part of the value.
                        unescaped_value.push(';');
                        chars = piece.chars()
                    } else {
                        break;
                    }
                }
                if !name_valid || !valid_value(value) {
                    continue;
                }
                unescaped_value
            } else {
                let value = value.trim_end_matches(http_whitespace);
                if value.is_empty() {
                    continue;
                }
                if !name_valid || !valid_value(value) {
                    continue;
                }
                value.to_owned()
            };
            parameters.push((name.to_ascii_lowercase(), value))
        }
    }
}