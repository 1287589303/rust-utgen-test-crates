fn float_digits(input: Cursor) -> Result<Cursor, Reject> {
    let mut chars = input.chars().peekable();
    match chars.next() {
        Some(ch) if '0' <= ch && ch <= '9' => {}
        _ => return Err(Reject),
    }

    let mut len = 1;
    let mut has_dot = false;
    let mut has_exp = false;
    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '_' => {
                chars.next();
                len += 1;
            }
            '.' => {
                if has_dot {
                    break;
                }
                chars.next();
                if chars
                    .peek()
                    .map_or(false, |&ch| ch == '.' || is_ident_start(ch))
                {
                    return Err(Reject);
                }
                len += 1;
                has_dot = true;
            }
            'e' | 'E' => {
                chars.next();
                len += 1;
                has_exp = true;
                break;
            }
            _ => break,
        }
    }

    if !(has_dot || has_exp) {
        return Err(Reject);
    }

    if has_exp {
        let token_before_exp = if has_dot {
            Ok(input.advance(len - 1))
        } else {
            Err(Reject)
        };
        let mut has_sign = false;
        let mut has_exp_value = false;
        while let Some(&ch) = chars.peek() {
            match ch {
                '+' | '-' => {
                    if has_exp_value {
                        break;
                    }
                    if has_sign {
                        return token_before_exp;
                    }
                    chars.next();
                    len += 1;
                    has_sign = true;
                }
                '0'..='9' => {
                    chars.next();
                    len += 1;
                    has_exp_value = true;
                }
                '_' => {
                    chars.next();
                    len += 1;
                }
                _ => break,
            }
        }
        if !has_exp_value {
            return token_before_exp;
        }
    }

    Ok(input.advance(len))
}