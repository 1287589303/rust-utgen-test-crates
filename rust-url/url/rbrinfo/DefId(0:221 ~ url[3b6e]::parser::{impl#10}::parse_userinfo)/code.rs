fn parse_userinfo<'i>(
        &mut self,
        mut input: Input<'i>,
        scheme_type: SchemeType,
    ) -> ParseResult<(u32, Input<'i>)> {
        let mut last_at = None;
        let mut remaining = input.clone();
        let mut char_count = 0;
        while let Some(c) = remaining.next() {
            match c {
                '@' => {
                    if last_at.is_some() {
                        self.log_violation(SyntaxViolation::UnencodedAtSign)
                    } else {
                        self.log_violation(SyntaxViolation::EmbeddedCredentials)
                    }
                    last_at = Some((char_count, remaining.clone()))
                }
                '/' | '?' | '#' => break,
                '\\' if scheme_type.is_special() => break,
                _ => (),
            }
            char_count += 1;
        }
        let (mut userinfo_char_count, remaining) = match last_at {
            None => return Ok((to_u32(self.serialization.len())?, input)),
            Some((0, remaining)) => {
                // Otherwise, if one of the following is true
                // c is the EOF code point, U+002F (/), U+003F (?), or U+0023 (#)
                // url is special and c is U+005C (\)
                // If @ flag is set and buffer is the empty string, validation error, return failure.
                if let (Some(c), _) = remaining.split_first() {
                    if c == '/' || c == '?' || c == '#' || (scheme_type.is_special() && c == '\\') {
                        return Err(ParseError::EmptyHost);
                    }
                }
                return Ok((to_u32(self.serialization.len())?, remaining));
            }
            Some(x) => x,
        };

        let mut username_end = None;
        let mut has_password = false;
        let mut has_username = false;
        while userinfo_char_count > 0 {
            let (c, utf8_c) = input.next_utf8().unwrap();
            userinfo_char_count -= 1;
            if c == ':' && username_end.is_none() {
                // Start parsing password
                username_end = Some(to_u32(self.serialization.len())?);
                // We don't add a colon if the password is empty
                if userinfo_char_count > 0 {
                    self.serialization.push(':');
                    has_password = true;
                }
            } else {
                if !has_password {
                    has_username = true;
                }
                self.check_url_code_point(c, &input);
                self.serialization
                    .extend(utf8_percent_encode(utf8_c, USERINFO));
            }
        }
        let username_end = match username_end {
            Some(i) => i,
            None => to_u32(self.serialization.len())?,
        };
        if has_username || has_password {
            self.serialization.push('@');
        }
        Ok((username_end, remaining))
    }