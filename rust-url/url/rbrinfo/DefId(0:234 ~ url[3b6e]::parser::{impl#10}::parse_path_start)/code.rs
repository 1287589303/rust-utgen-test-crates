pub fn parse_path_start<'i>(
        &mut self,
        scheme_type: SchemeType,
        has_host: &mut bool,
        input: Input<'i>,
    ) -> Input<'i> {
        let path_start = self.serialization.len();
        let (maybe_c, remaining) = input.split_first();
        // If url is special, then:
        if scheme_type.is_special() {
            if maybe_c == Some('\\') {
                // If c is U+005C (\), validation error.
                self.log_violation(SyntaxViolation::Backslash);
            }
            // A special URL always has a non-empty path.
            if !self.serialization.ends_with('/') {
                self.serialization.push('/');
                // We have already made sure the forward slash is present.
                if maybe_c == Some('/') || maybe_c == Some('\\') {
                    return self.parse_path(scheme_type, has_host, path_start, remaining);
                }
            }
            return self.parse_path(scheme_type, has_host, path_start, input);
        } else if maybe_c == Some('?') || maybe_c == Some('#') {
            // Otherwise, if state override is not given and c is U+003F (?),
            // set url’s query to the empty string and state to query state.
            // Otherwise, if state override is not given and c is U+0023 (#),
            // set url’s fragment to the empty string and state to fragment state.
            // The query and path states will be handled by the caller.
            return input;
        }

        if maybe_c.is_some() && maybe_c != Some('/') {
            self.serialization.push('/');
        }
        // Otherwise, if c is not the EOF code point:
        self.parse_path(scheme_type, has_host, path_start, input)
    }