fn parse_class(&self) -> Result<Hir, Error> {
        assert_eq!(self.char(), '[');

        let mut union = vec![];
        if !self.bump_and_bump_space() {
            return Err(Error::new(ERR_CLASS_UNCLOSED));
        }
        // Determine whether the class is negated or not.
        let negate = if self.char() != '^' {
            false
        } else {
            if !self.bump_and_bump_space() {
                return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_NEGATION));
            }
            true
        };
        // Accept any number of `-` as literal `-`.
        while self.char() == '-' {
            union.push(hir::ClassRange { start: '-', end: '-' });
            if !self.bump_and_bump_space() {
                return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_DASH));
            }
        }
        // If `]` is the *first* char in a set, then interpret it as a literal
        // `]`. That is, an empty class is impossible to write.
        if union.is_empty() && self.char() == ']' {
            union.push(hir::ClassRange { start: ']', end: ']' });
            if !self.bump_and_bump_space() {
                return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_CLOSING));
            }
        }
        loop {
            self.bump_space();
            if self.is_done() {
                return Err(Error::new(ERR_CLASS_UNCLOSED));
            }
            match self.char() {
                '[' => {
                    // Attempt to treat this as the beginning of a POSIX class.
                    // If POSIX class parsing fails, then the parser backs up
                    // to `[`.
                    if let Some(class) = self.maybe_parse_posix_class() {
                        union.extend_from_slice(&class.ranges);
                        continue;
                    }
                    // ... otherwise we don't support nested classes.
                    return Err(Error::new(ERR_CLASS_NEST_UNSUPPORTED));
                }
                ']' => {
                    self.bump();
                    let mut class = hir::Class::new(union);
                    // Note that we must apply case folding before negation!
                    // Consider `(?i)[^x]`. If we applied negation first, then
                    // the result would be the character class that matched any
                    // Unicode scalar value.
                    if self.flags().case_insensitive {
                        class.ascii_case_fold();
                    }
                    if negate {
                        class.negate();
                    }
                    return Ok(Hir::class(class));
                }
                '&' if self.peek() == Some('&') => {
                    return Err(Error::new(
                        ERR_CLASS_INTERSECTION_UNSUPPORTED,
                    ));
                }
                '-' if self.peek() == Some('-') => {
                    return Err(Error::new(ERR_CLASS_DIFFERENCE_UNSUPPORTED));
                }
                '~' if self.peek() == Some('~') => {
                    return Err(Error::new(
                        ERR_CLASS_SYMDIFFERENCE_UNSUPPORTED,
                    ));
                }
                _ => self.parse_class_range(&mut union)?,
            }
        }
    }