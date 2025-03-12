fn parse_capture_name(&self) -> Result<&str, Error> {
        if self.is_done() {
            return Err(Error::new(ERR_MISSING_GROUP_NAME));
        }
        let start = self.pos();
        loop {
            if self.char() == '>' {
                break;
            }
            if !is_capture_char(self.char(), self.pos() == start) {
                return Err(Error::new(ERR_INVALID_GROUP_NAME));
            }
            if !self.bump() {
                break;
            }
        }
        let end = self.pos();
        if self.is_done() {
            return Err(Error::new(ERR_UNCLOSED_GROUP_NAME));
        }
        assert_eq!(self.char(), '>');
        self.bump();
        let name = &self.pattern()[start..end];
        if name.is_empty() {
            return Err(Error::new(ERR_EMPTY_GROUP_NAME));
        }
        self.add_capture_name(name)?;
        Ok(name)
    }