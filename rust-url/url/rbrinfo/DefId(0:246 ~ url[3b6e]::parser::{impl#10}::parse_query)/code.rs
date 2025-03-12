pub fn parse_query<'i>(
        &mut self,
        scheme_type: SchemeType,
        scheme_end: u32,
        input: Input<'i>,
    ) -> Option<Input<'i>> {
        struct QueryPartIter<'i, 'p> {
            is_url_parser: bool,
            input: Input<'i>,
            violation_fn: Option<&'p dyn Fn(SyntaxViolation)>,
        }

        impl<'i> Iterator for QueryPartIter<'i, '_> {
            type Item = (&'i str, bool);

            fn next(&mut self) -> Option<Self::Item> {
                let start = self.input.chars.as_str();
                // bypass self.input.next() in order to get string slices
                // which are faster to operate on
                while let Some(c) = self.input.chars.next() {
                    match c {
                        ascii_tab_or_new_line_pattern!() => {
                            return Some((
                                &start[..start.len() - self.input.chars.as_str().len() - 1],
                                false,
                            ));
                        }
                        '#' if self.is_url_parser => {
                            return Some((
                                &start[..start.len() - self.input.chars.as_str().len() - 1],
                                true,
                            ));
                        }
                        c => {
                            if let Some(vfn) = &self.violation_fn {
                                check_url_code_point(vfn, c, &self.input);
                            }
                        }
                    }
                }
                if start.is_empty() {
                    None
                } else {
                    Some((start, false))
                }
            }
        }

        let mut part_iter = QueryPartIter {
            is_url_parser: self.context == Context::UrlParser,
            input,
            violation_fn: self.violation_fn,
        };
        let set = if scheme_type.is_special() {
            SPECIAL_QUERY
        } else {
            QUERY
        };
        let query_encoding_override = self.query_encoding_override.filter(|_| {
            matches!(
                &self.serialization[..scheme_end as usize],
                "http" | "https" | "file" | "ftp"
            )
        });

        while let Some((part, is_finished)) = part_iter.next() {
            match query_encoding_override {
                // slightly faster to be repetitive and not convert text to Cow
                Some(o) => self.serialization.extend(percent_encode(&o(part), set)),
                None => self
                    .serialization
                    .extend(percent_encode(part.as_bytes(), set)),
            }
            if is_finished {
                return Some(part_iter.input);
            }
        }

        None
    }