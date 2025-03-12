fn maybe_parse_special_word_boundary(
        &self,
        wb_start: Position,
    ) -> Result<Option<ast::AssertionKind>> {
        assert_eq!(self.char(), '{');

        let is_valid_char = |c| match c {
            'A'..='Z' | 'a'..='z' | '-' => true,
            _ => false,
        };
        let start = self.pos();
        if !self.bump_and_bump_space() {
            return Err(self.error(
                Span::new(wb_start, self.pos()),
                ast::ErrorKind::SpecialWordOrRepetitionUnexpectedEof,
            ));
        }
        let start_contents = self.pos();
        // This is one of the critical bits: if the first non-whitespace
        // character isn't in [-A-Za-z] (i.e., this can't be a special word
        // boundary), then we bail and let the counted repetition parser deal
        // with this.
        if !is_valid_char(self.char()) {
            self.parser().pos.set(start);
            return Ok(None);
        }

        // Now collect up our chars until we see a '}'.
        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();
        while !self.is_eof() && is_valid_char(self.char()) {
            scratch.push(self.char());
            self.bump_and_bump_space();
        }
        if self.is_eof() || self.char() != '}' {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::SpecialWordBoundaryUnclosed,
            ));
        }
        let end = self.pos();
        self.bump();
        let kind = match scratch.as_str() {
            "start" => ast::AssertionKind::WordBoundaryStart,
            "end" => ast::AssertionKind::WordBoundaryEnd,
            "start-half" => ast::AssertionKind::WordBoundaryStartHalf,
            "end-half" => ast::AssertionKind::WordBoundaryEndHalf,
            _ => {
                return Err(self.error(
                    Span::new(start_contents, end),
                    ast::ErrorKind::SpecialWordBoundaryUnrecognized,
                ))
            }
        };
        Ok(Some(kind))
    }