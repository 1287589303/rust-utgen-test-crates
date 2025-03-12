pub fn crlf(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.crlf = if yes { Some(true) } else { None };
        self
    }