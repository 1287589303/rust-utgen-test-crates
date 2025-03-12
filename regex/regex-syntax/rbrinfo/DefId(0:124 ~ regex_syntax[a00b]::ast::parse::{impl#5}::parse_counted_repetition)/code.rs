fn parse_counted_repetition(
        &self,
        mut concat: ast::Concat,
    ) -> Result<ast::Concat> {
        assert!(self.char() == '{');
        let start = self.pos();
        let ast = match concat.asts.pop() {
            Some(ast) => ast,
            None => {
                return Err(
                    self.error(self.span(), ast::ErrorKind::RepetitionMissing)
                )
            }
        };
        match ast {
            Ast::Empty(_) | Ast::Flags(_) => {
                return Err(
                    self.error(self.span(), ast::ErrorKind::RepetitionMissing)
                )
            }
            _ => {}
        }
        if !self.bump_and_bump_space() {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }
        let count_start = specialize_err(
            self.parse_decimal(),
            ast::ErrorKind::DecimalEmpty,
            ast::ErrorKind::RepetitionCountDecimalEmpty,
        );
        if self.is_eof() {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }
        let range = if self.char() == ',' {
            if !self.bump_and_bump_space() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::RepetitionCountUnclosed,
                ));
            }
            if self.char() != '}' {
                let count_start = match count_start {
                    Ok(c) => c,
                    Err(err)
                        if err.kind
                            == ast::ErrorKind::RepetitionCountDecimalEmpty =>
                    {
                        if self.parser().empty_min_range {
                            0
                        } else {
                            return Err(err);
                        }
                    }
                    err => err?,
                };
                let count_end = specialize_err(
                    self.parse_decimal(),
                    ast::ErrorKind::DecimalEmpty,
                    ast::ErrorKind::RepetitionCountDecimalEmpty,
                )?;
                ast::RepetitionRange::Bounded(count_start, count_end)
            } else {
                ast::RepetitionRange::AtLeast(count_start?)
            }
        } else {
            ast::RepetitionRange::Exactly(count_start?)
        };

        if self.is_eof() || self.char() != '}' {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }

        let mut greedy = true;
        if self.bump_and_bump_space() && self.char() == '?' {
            greedy = false;
            self.bump();
        }

        let op_span = Span::new(start, self.pos());
        if !range.is_valid() {
            return Err(
                self.error(op_span, ast::ErrorKind::RepetitionCountInvalid)
            );
        }
        concat.asts.push(Ast::repetition(ast::Repetition {
            span: ast.span().with_end(self.pos()),
            op: ast::RepetitionOp {
                span: op_span,
                kind: ast::RepetitionKind::Range(range),
            },
            greedy,
            ast: Box::new(ast),
        }));
        Ok(concat)
    }