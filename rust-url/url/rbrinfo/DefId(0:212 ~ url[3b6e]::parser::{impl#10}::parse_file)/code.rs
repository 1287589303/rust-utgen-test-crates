fn parse_file(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        base_file_url: Option<&Url>,
    ) -> ParseResult<Url> {
        use crate::SyntaxViolation::Backslash;
        // file state
        debug_assert!(self.serialization.is_empty());
        let (first_char, input_after_first_char) = input.split_first();
        if matches!(first_char, Some('/') | Some('\\')) {
            self.log_violation_if(SyntaxViolation::Backslash, || first_char == Some('\\'));
            // file slash state
            let (next_char, input_after_next_char) = input_after_first_char.split_first();
            if matches!(next_char, Some('/') | Some('\\')) {
                self.log_violation_if(Backslash, || next_char == Some('\\'));
                // file host state
                self.serialization.push_str("file://");
                let scheme_end = "file".len() as u32;
                let host_start = "file://".len() as u32;
                let (path_start, mut host, remaining) =
                    self.parse_file_host(input_after_next_char)?;
                let mut host_end = to_u32(self.serialization.len())?;
                let mut has_host = !matches!(host, HostInternal::None);
                let remaining = if path_start {
                    self.parse_path_start(SchemeType::File, &mut has_host, remaining)
                } else {
                    let path_start = self.serialization.len();
                    self.serialization.push('/');
                    self.parse_path(SchemeType::File, &mut has_host, path_start, remaining)
                };

                // For file URLs that have a host and whose path starts
                // with the windows drive letter we just remove the host.
                if !has_host {
                    self.serialization
                        .drain(host_start as usize..host_end as usize);
                    host_end = host_start;
                    host = HostInternal::None;
                }
                let (query_start, fragment_start) =
                    self.parse_query_and_fragment(scheme_type, scheme_end, remaining)?;
                return Ok(Url {
                    serialization: self.serialization,
                    scheme_end,
                    username_end: host_start,
                    host_start,
                    host_end,
                    host,
                    port: None,
                    path_start: host_end,
                    query_start,
                    fragment_start,
                });
            } else {
                self.serialization.push_str("file://");
                let scheme_end = "file".len() as u32;
                let host_start = "file://".len();
                let mut host_end = host_start;
                let mut host = HostInternal::None;
                if !starts_with_windows_drive_letter_segment(&input_after_first_char) {
                    if let Some(base_url) = base_file_url {
                        let first_segment = base_url.path_segments().unwrap().next().unwrap();
                        if is_normalized_windows_drive_letter(first_segment) {
                            self.serialization.push('/');
                            self.serialization.push_str(first_segment);
                        } else if let Some(host_str) = base_url.host_str() {
                            self.serialization.push_str(host_str);
                            host_end = self.serialization.len();
                            host = base_url.host;
                        }
                    }
                }
                // If c is the EOF code point, U+002F (/), U+005C (\), U+003F (?), or U+0023 (#), then decrease pointer by one
                let parse_path_input = if let Some(c) = first_char {
                    if c == '/' || c == '\\' || c == '?' || c == '#' {
                        input
                    } else {
                        input_after_first_char
                    }
                } else {
                    input_after_first_char
                };

                let remaining =
                    self.parse_path(SchemeType::File, &mut false, host_end, parse_path_input);

                let host_start = host_start as u32;

                let (query_start, fragment_start) =
                    self.parse_query_and_fragment(scheme_type, scheme_end, remaining)?;

                let host_end = host_end as u32;
                return Ok(Url {
                    serialization: self.serialization,
                    scheme_end,
                    username_end: host_start,
                    host_start,
                    host_end,
                    host,
                    port: None,
                    path_start: host_end,
                    query_start,
                    fragment_start,
                });
            }
        }
        if let Some(base_url) = base_file_url {
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
                _ => {
                    if !starts_with_windows_drive_letter_segment(&input) {
                        let before_query = match (base_url.query_start, base_url.fragment_start) {
                            (None, None) => &*base_url.serialization,
                            (Some(i), _) | (None, Some(i)) => base_url.slice(..i),
                        };
                        self.serialization.push_str(before_query);
                        self.shorten_path(SchemeType::File, base_url.path_start as usize);
                        let remaining = self.parse_path(
                            SchemeType::File,
                            &mut true,
                            base_url.path_start as usize,
                            input,
                        );
                        self.with_query_and_fragment(
                            SchemeType::File,
                            base_url.scheme_end,
                            base_url.username_end,
                            base_url.host_start,
                            base_url.host_end,
                            base_url.host,
                            base_url.port,
                            base_url.path_start,
                            remaining,
                        )
                    } else {
                        self.serialization.push_str("file:///");
                        let scheme_end = "file".len() as u32;
                        let path_start = "file://".len();
                        let remaining =
                            self.parse_path(SchemeType::File, &mut false, path_start, input);
                        let (query_start, fragment_start) =
                            self.parse_query_and_fragment(SchemeType::File, scheme_end, remaining)?;
                        let path_start = path_start as u32;
                        Ok(Url {
                            serialization: self.serialization,
                            scheme_end,
                            username_end: path_start,
                            host_start: path_start,
                            host_end: path_start,
                            host: HostInternal::None,
                            port: None,
                            path_start,
                            query_start,
                            fragment_start,
                        })
                    }
                }
            }
        } else {
            self.serialization.push_str("file:///");
            let scheme_end = "file".len() as u32;
            let path_start = "file://".len();
            let remaining = self.parse_path(SchemeType::File, &mut false, path_start, input);
            let (query_start, fragment_start) =
                self.parse_query_and_fragment(SchemeType::File, scheme_end, remaining)?;
            let path_start = path_start as u32;
            Ok(Url {
                serialization: self.serialization,
                scheme_end,
                username_end: path_start,
                host_start: path_start,
                host_end: path_start,
                host: HostInternal::None,
                port: None,
                path_start,
                query_start,
                fragment_start,
            })
        }
    }