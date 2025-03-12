fn parse_group(&self) -> Result<Either<ast::SetFlags, ast::Group>> {
        assert_eq!(self.char(), '(');
        let open_span = self.span_char();
        self.bump();
        self.bump_space();
        if self.is_lookaround_prefix() {
            return Err(self.error(
                Span::new(open_span.start, self.span().end),
                ast::ErrorKind::UnsupportedLookAround,
            ));
        }
        let inner_span = self.span();
        let mut starts_with_p = true;
        if self.bump_if("?P<") || {
            starts_with_p = false;
            self.bump_if("?<")
        } {
            let capture_index = self.next_capture_index(open_span)?;
            let name = self.parse_capture_name(capture_index)?;
            Ok(Either::Right(ast::Group {
                span: open_span,
                kind: ast::GroupKind::CaptureName { starts_with_p, name },
                ast: Box::new(Ast::empty(self.span())),
            }))
        } else if self.bump_if("?") {
            if self.is_eof() {
                return Err(
                    self.error(open_span, ast::ErrorKind::GroupUnclosed)
                );
            }
            let flags = self.parse_flags()?;
            let char_end = self.char();
            self.bump();
            if char_end == ')' {
                // We don't allow empty flags, e.g., `(?)`. We instead
                // interpret it as a repetition operator missing its argument.
                if flags.items.is_empty() {
                    return Err(self.error(
                        inner_span,
                        ast::ErrorKind::RepetitionMissing,
                    ));
                }
                Ok(Either::Left(ast::SetFlags {
                    span: Span { end: self.pos(), ..open_span },
                    flags,
                }))
            } else {
                assert_eq!(char_end, ':');
                Ok(Either::Right(ast::Group {
                    span: open_span,
                    kind: ast::GroupKind::NonCapturing(flags),
                    ast: Box::new(Ast::empty(self.span())),
                }))
            }
        } else {
            let capture_index = self.next_capture_index(open_span)?;
            Ok(Either::Right(ast::Group {
                span: open_span,
                kind: ast::GroupKind::CaptureIndex(capture_index),
                ast: Box::new(Ast::empty(self.span())),
            }))
        }
    }