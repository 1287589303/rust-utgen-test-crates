fn parse_relative(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        base_url: &Url,
    ) -> ParseResult<Url> {
        // relative state
        debug_assert!(self.serialization.is_empty());
        let (first_char, input_after_first_char) = input.split_first();
        match first_char {
            None => {
                // Copy everything except the fragment
                let before_fragment = match base_url.fragment_start {
                    Some(i) => &base_url.serialization[..i as usize],
                    None => &*base_url.serialization,
                };
                self.serialization.push_str(before_fragment);
                Ok(Url {
                    serialization: self.serialization,
                    fragment_start: None,
                    ..*base_url
                })
            }
            Some('?') => {
                // Copy everything up to the query string
                let before_query = match (base_url.query_start, base_url.fragment_start) {
                    (None, None) => &*base_url.serialization,
                    (Some(i), _) | (None, Some(i)) => base_url.slice(..i),
                };
                self.serialization.push_str(before_query);
                let (query_start, fragment_start) =
                    self.parse_query_and_fragment(scheme_type, base_url.scheme_end, input)?;
                Ok(Url {
                    serialization: self.serialization,
                    query_start,
                    fragment_start,
                    ..*base_url
                })
            }
            Some('#') => self.fragment_only(base_url, input),
            Some('/') | Some('\\') => {
                let (slashes_count, remaining) = input.count_matching(|c| matches!(c, '/' | '\\'));
                if slashes_count >= 2 {
                    self.log_violation_if(SyntaxViolation::ExpectedDoubleSlash, || {
                        input
                            .clone()
                            .take_while(|&c| matches!(c, '/' | '\\'))
                            .collect::<String>()
                            != "//"
                    });
                    let scheme_end = base_url.scheme_end;
                    debug_assert!(base_url.byte_at(scheme_end) == b':');
                    self.serialization
                        .push_str(base_url.slice(..scheme_end + 1));
                    if let Some(after_prefix) = input.split_prefix("//") {
                        return self.after_double_slash(after_prefix, scheme_type, scheme_end);
                    }
                    return self.after_double_slash(remaining, scheme_type, scheme_end);
                }
                let path_start = base_url.path_start;
                self.serialization.push_str(base_url.slice(..path_start));
                self.serialization.push('/');
                let remaining = self.parse_path(
                    scheme_type,
                    &mut true,
                    path_start as usize,
                    input_after_first_char,
                );
                self.with_query_and_fragment(
                    scheme_type,
                    base_url.scheme_end,
                    base_url.username_end,
                    base_url.host_start,
                    base_url.host_end,
                    base_url.host,
                    base_url.port,
                    base_url.path_start,
                    remaining,
                )
            }
            _ => {
                let before_query = match (base_url.query_start, base_url.fragment_start) {
                    (None, None) => &*base_url.serialization,
                    (Some(i), _) | (None, Some(i)) => base_url.slice(..i),
                };
                self.serialization.push_str(before_query);
                // FIXME spec says just "remove last entry", not the "pop" algorithm
                self.pop_path(scheme_type, base_url.path_start as usize);
                // A special url always has a path.
                // A path always starts with '/'
                if self.serialization.len() == base_url.path_start as usize
                    && (SchemeType::from(base_url.scheme()).is_special() || !input.is_empty())
                {
                    self.serialization.push('/');
                }
                let remaining = match input.split_first() {
                    (Some('/'), remaining) => self.parse_path(
                        scheme_type,
                        &mut true,
                        base_url.path_start as usize,
                        remaining,
                    ),
                    _ => {
                        self.parse_path(scheme_type, &mut true, base_url.path_start as usize, input)
                    }
                };
                self.with_query_and_fragment(
                    scheme_type,
                    base_url.scheme_end,
                    base_url.username_end,
                    base_url.host_start,
                    base_url.host_end,
                    base_url.host,
                    base_url.port,
                    base_url.path_start,
                    remaining,
                )
            }
        }
    }