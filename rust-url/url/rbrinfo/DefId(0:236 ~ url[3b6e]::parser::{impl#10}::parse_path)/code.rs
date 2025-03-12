pub fn parse_path<'i>(
        &mut self,
        scheme_type: SchemeType,
        has_host: &mut bool,
        path_start: usize,
        mut input: Input<'i>,
    ) -> Input<'i> {
        // it's much faster to call utf8_percent_encode in bulk
        fn push_pending(
            serialization: &mut String,
            start_str: &str,
            remaining_len: usize,
            context: Context,
            scheme_type: SchemeType,
        ) {
            let text = &start_str[..start_str.len() - remaining_len];
            if text.is_empty() {
                return;
            }
            if context == Context::PathSegmentSetter {
                if scheme_type.is_special() {
                    serialization.extend(utf8_percent_encode(text, SPECIAL_PATH_SEGMENT));
                } else {
                    serialization.extend(utf8_percent_encode(text, PATH_SEGMENT));
                }
            } else {
                serialization.extend(utf8_percent_encode(text, PATH));
            }
        }

        // Relative path state
        loop {
            let mut segment_start = self.serialization.len();
            let mut ends_with_slash = false;
            let mut start_str = input.chars.as_str();
            loop {
                let input_before_c = input.clone();
                // bypass input.next() and manually handle ascii_tab_or_new_line
                // in order to encode string slices in bulk
                let c = if let Some(c) = input.chars.next() {
                    c
                } else {
                    push_pending(
                        &mut self.serialization,
                        start_str,
                        0,
                        self.context,
                        scheme_type,
                    );
                    break;
                };
                match c {
                    ascii_tab_or_new_line_pattern!() => {
                        push_pending(
                            &mut self.serialization,
                            start_str,
                            input_before_c.chars.as_str().len(),
                            self.context,
                            scheme_type,
                        );
                        start_str = input.chars.as_str();
                    }
                    '/' if self.context != Context::PathSegmentSetter => {
                        push_pending(
                            &mut self.serialization,
                            start_str,
                            input_before_c.chars.as_str().len(),
                            self.context,
                            scheme_type,
                        );
                        self.serialization.push(c);
                        ends_with_slash = true;
                        break;
                    }
                    '\\' if self.context != Context::PathSegmentSetter
                        && scheme_type.is_special() =>
                    {
                        push_pending(
                            &mut self.serialization,
                            start_str,
                            input_before_c.chars.as_str().len(),
                            self.context,
                            scheme_type,
                        );
                        self.log_violation(SyntaxViolation::Backslash);
                        self.serialization.push('/');
                        ends_with_slash = true;
                        break;
                    }
                    '?' | '#' if self.context == Context::UrlParser => {
                        push_pending(
                            &mut self.serialization,
                            start_str,
                            input_before_c.chars.as_str().len(),
                            self.context,
                            scheme_type,
                        );
                        input = input_before_c;
                        break;
                    }
                    _ => {
                        self.check_url_code_point(c, &input);
                        if scheme_type.is_file()
                            && self.serialization.len() > path_start
                            && is_normalized_windows_drive_letter(
                                &self.serialization[path_start + 1..],
                            )
                        {
                            push_pending(
                                &mut self.serialization,
                                start_str,
                                input_before_c.chars.as_str().len(),
                                self.context,
                                scheme_type,
                            );
                            start_str = input_before_c.chars.as_str();
                            self.serialization.push('/');
                            segment_start += 1;
                        }
                    }
                }
            }

            let segment_before_slash = if ends_with_slash {
                &self.serialization[segment_start..self.serialization.len() - 1]
            } else {
                &self.serialization[segment_start..self.serialization.len()]
            };
            match segment_before_slash {
                // If buffer is a double-dot path segment, shorten url’s path,
                ".." | "%2e%2e" | "%2e%2E" | "%2E%2e" | "%2E%2E" | "%2e." | "%2E." | ".%2e"
                | ".%2E" => {
                    debug_assert!(self.serialization.as_bytes()[segment_start - 1] == b'/');
                    self.serialization.truncate(segment_start);
                    if self.serialization.ends_with('/')
                        && Parser::last_slash_can_be_removed(&self.serialization, path_start)
                    {
                        self.serialization.pop();
                    }
                    self.shorten_path(scheme_type, path_start);

                    // and then if neither c is U+002F (/), nor url is special and c is U+005C (\), append the empty string to url’s path.
                    if ends_with_slash && !self.serialization.ends_with('/') {
                        self.serialization.push('/');
                    }
                }
                // Otherwise, if buffer is a single-dot path segment and if neither c is U+002F (/),
                // nor url is special and c is U+005C (\), append the empty string to url’s path.
                "." | "%2e" | "%2E" => {
                    self.serialization.truncate(segment_start);
                    if !self.serialization.ends_with('/') {
                        self.serialization.push('/');
                    }
                }
                _ => {
                    // If url’s scheme is "file", url’s path is empty, and buffer is a Windows drive letter, then
                    if scheme_type.is_file()
                        && segment_start == path_start + 1
                        && is_windows_drive_letter(segment_before_slash)
                    {
                        // Replace the second code point in buffer with U+003A (:).
                        if let Some(c) = segment_before_slash.chars().next() {
                            self.serialization.truncate(segment_start);
                            self.serialization.push(c);
                            self.serialization.push(':');
                            if ends_with_slash {
                                self.serialization.push('/');
                            }
                        }
                        // If url’s host is neither the empty string nor null,
                        // validation error, set url’s host to the empty string.
                        if *has_host {
                            self.log_violation(SyntaxViolation::FileWithHostAndWindowsDrive);
                            *has_host = false; // FIXME account for this in callers
                        }
                    }
                }
            }
            if !ends_with_slash {
                break;
            }
        }
        if scheme_type.is_file() {
            // while url’s path’s size is greater than 1
            // and url’s path[0] is the empty string,
            // validation error, remove the first item from url’s path.
            //FIXME: log violation
            let path = self.serialization.split_off(path_start);
            self.serialization.push('/');
            self.serialization.push_str(path.trim_start_matches('/'));
        }

        input
    }