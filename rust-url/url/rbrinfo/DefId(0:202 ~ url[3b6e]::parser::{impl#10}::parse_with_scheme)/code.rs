fn parse_with_scheme(mut self, input: Input<'_>) -> ParseResult<Url> {
        use crate::SyntaxViolation::{ExpectedDoubleSlash, ExpectedFileDoubleSlash};
        let scheme_end = to_u32(self.serialization.len())?;
        let scheme_type = SchemeType::from(&self.serialization);
        self.serialization.push(':');
        match scheme_type {
            SchemeType::File => {
                self.log_violation_if(ExpectedFileDoubleSlash, || !input.starts_with("//"));
                let base_file_url = self.base_url.and_then(|base| {
                    if base.scheme() == "file" {
                        Some(base)
                    } else {
                        None
                    }
                });
                self.serialization.clear();
                self.parse_file(input, scheme_type, base_file_url)
            }
            SchemeType::SpecialNotFile => {
                // special relative or authority state
                let (slashes_count, remaining) = input.count_matching(|c| matches!(c, '/' | '\\'));
                if let Some(base_url) = self.base_url {
                    if slashes_count < 2
                        && base_url.scheme() == &self.serialization[..scheme_end as usize]
                    {
                        // "Cannot-be-a-base" URLs only happen with "not special" schemes.
                        debug_assert!(!base_url.cannot_be_a_base());
                        self.serialization.clear();
                        return self.parse_relative(input, scheme_type, base_url);
                    }
                }
                // special authority slashes state
                self.log_violation_if(ExpectedDoubleSlash, || {
                    input
                        .clone()
                        .take_while(|&c| matches!(c, '/' | '\\'))
                        .collect::<String>()
                        != "//"
                });
                self.after_double_slash(remaining, scheme_type, scheme_end)
            }
            SchemeType::NotSpecial => self.parse_non_special(input, scheme_type, scheme_end),
        }
    }