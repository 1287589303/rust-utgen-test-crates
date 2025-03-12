fn parse_group(&self) -> Result<Option<Hir>, Error> {
        assert_eq!(self.char(), '(');
        self.bump_and_bump_space();
        if self.is_lookaround_prefix() {
            return Err(Error::new(ERR_LOOK_UNSUPPORTED));
        }
        if self.bump_if("?P<") || self.bump_if("?<") {
            let index = self.next_capture_index()?;
            let name = Some(Box::from(self.parse_capture_name()?));
            let sub = Box::new(self.parse_inner()?);
            let cap = hir::Capture { index, name, sub };
            Ok(Some(Hir::capture(cap)))
        } else if self.bump_if("?") {
            if self.is_done() {
                return Err(Error::new(ERR_UNCLOSED_GROUP_QUESTION));
            }
            let start = self.pos();
            // The flags get reset in the top-level 'parse' routine.
            *self.flags.borrow_mut() = self.parse_flags()?;
            let consumed = self.pos() - start;
            if self.char() == ')' {
                // We don't allow empty flags, e.g., `(?)`.
                if consumed == 0 {
                    return Err(Error::new(ERR_EMPTY_FLAGS));
                }
                Ok(None)
            } else {
                assert_eq!(':', self.char());
                self.bump();
                self.parse_inner().map(Some)
            }
        } else {
            let index = self.next_capture_index()?;
            let sub = Box::new(self.parse_inner()?);
            let cap = hir::Capture { index, name: None, sub };
            Ok(Some(Hir::capture(cap)))
        }
    }