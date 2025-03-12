pub fn parse_url(mut self, input: &str) -> ParseResult<Url> {
        let input = Input::new_trim_c0_control_and_space(input, self.violation_fn);
        if let Ok(remaining) = self.parse_scheme(input.clone()) {
            return self.parse_with_scheme(remaining);
        }

        // No-scheme state
        if let Some(base_url) = self.base_url {
            if input.starts_with('#') {
                self.fragment_only(base_url, input)
            } else if base_url.cannot_be_a_base() {
                Err(ParseError::RelativeUrlWithCannotBeABaseBase)
            } else {
                let scheme_type = SchemeType::from(base_url.scheme());
                if scheme_type.is_file() {
                    self.parse_file(input, scheme_type, Some(base_url))
                } else {
                    self.parse_relative(input, scheme_type, base_url)
                }
            }
        } else {
            Err(ParseError::RelativeUrlWithoutBase)
        }
    }