fn maybe_parse_special_word_boundary(&self) -> Result<Option<Hir>, Error> {
        assert_eq!(self.char(), '{');

        let is_valid_char = |c| match c {
            'A'..='Z' | 'a'..='z' | '-' => true,
            _ => false,
        };
        let start = self.pos();
        if !self.bump_and_bump_space() {
            return Err(Error::new(ERR_SPECIAL_WORD_OR_REP_UNEXPECTED_EOF));
        }
        // This is one of the critical bits: if the first non-whitespace
        // character isn't in [-A-Za-z] (i.e., this can't be a special word
        // boundary), then we bail and let the counted repetition parser deal
        // with this.
        if !is_valid_char(self.char()) {
            self.pos.set(start);
            self.char.set(Some('{'));
            return Ok(None);
        }

        // Now collect up our chars until we see a '}'.
        let mut scratch = String::new();
        while !self.is_done() && is_valid_char(self.char()) {
            scratch.push(self.char());
            self.bump_and_bump_space();
        }
        if self.is_done() || self.char() != '}' {
            return Err(Error::new(ERR_SPECIAL_WORD_BOUNDARY_UNCLOSED));
        }
        self.bump();
        let kind = match scratch.as_str() {
            "start" => hir::Look::WordStart,
            "end" => hir::Look::WordEnd,
            "start-half" => hir::Look::WordStartHalf,
            "end-half" => hir::Look::WordEndHalf,
            _ => {
                return Err(Error::new(ERR_SPECIAL_WORD_BOUNDARY_UNRECOGNIZED))
            }
        };
        Ok(Some(Hir::look(kind)))
    }