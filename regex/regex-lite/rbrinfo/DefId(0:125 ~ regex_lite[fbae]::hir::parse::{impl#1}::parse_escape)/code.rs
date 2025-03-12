fn parse_escape(&self) -> Result<Hir, Error> {
        if self.is_done() {
            return Err(Error::new(ERR_ESCAPE_UNEXPECTED_EOF));
        }
        let ch = self.char();
        // Put some of the more complicated routines into helpers.
        match ch {
            '0'..='9' => return Err(Error::new(ERR_BACKREF_UNSUPPORTED)),
            'p' | 'P' => {
                return Err(Error::new(ERR_UNICODE_CLASS_UNSUPPORTED))
            }
            'x' | 'u' | 'U' => return self.parse_hex(),
            'd' | 's' | 'w' | 'D' | 'S' | 'W' => {
                return Ok(self.parse_perl_class());
            }
            _ => {}
        }

        // Handle all of the one letter sequences inline.
        self.bump();
        if hir::is_meta_character(ch) || hir::is_escapeable_character(ch) {
            return Ok(self.hir_char(ch));
        }
        let special = |ch| Ok(self.hir_char(ch));
        match ch {
            'a' => special('\x07'),
            'f' => special('\x0C'),
            't' => special('\t'),
            'n' => special('\n'),
            'r' => special('\r'),
            'v' => special('\x0B'),
            'A' => Ok(Hir::look(hir::Look::Start)),
            'z' => Ok(Hir::look(hir::Look::End)),
            'b' => {
                let mut hir = Hir::look(hir::Look::Word);
                if !self.is_done() && self.char() == '{' {
                    if let Some(special) =
                        self.maybe_parse_special_word_boundary()?
                    {
                        hir = special;
                    }
                }
                Ok(hir)
            }
            'B' => Ok(Hir::look(hir::Look::WordNegate)),
            '<' => Ok(Hir::look(hir::Look::WordStart)),
            '>' => Ok(Hir::look(hir::Look::WordEnd)),
            _ => Err(Error::new(ERR_ESCAPE_UNRECOGNIZED)),
        }
    }