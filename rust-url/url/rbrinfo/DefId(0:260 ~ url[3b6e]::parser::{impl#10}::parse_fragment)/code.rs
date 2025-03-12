pub fn parse_fragment(&mut self, input: Input<'_>) {
        struct FragmentPartIter<'i, 'p> {
            input: Input<'i>,
            violation_fn: Option<&'p dyn Fn(SyntaxViolation)>,
        }

        impl<'i> Iterator for FragmentPartIter<'i, '_> {
            type Item = &'i str;

            fn next(&mut self) -> Option<Self::Item> {
                let start = self.input.chars.as_str();
                // bypass self.input.next() in order to get string slices
                // which are faster to operate on
                while let Some(c) = self.input.chars.next() {
                    match c {
                        ascii_tab_or_new_line_pattern!() => {
                            return Some(
                                &start[..start.len() - self.input.chars.as_str().len() - 1],
                            );
                        }
                        '\0' => {
                            if let Some(vfn) = &self.violation_fn {
                                vfn(SyntaxViolation::NullInFragment);
                            }
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
                    Some(start)
                }
            }
        }

        let part_iter = FragmentPartIter {
            input,
            violation_fn: self.violation_fn,
        };

        for part in part_iter {
            self.serialization
                .extend(utf8_percent_encode(part, FRAGMENT));
        }
    }