pub(super) fn parse(&self) -> Result<Hir, Error> {
        let hir = self.parse_inner()?;
        // While we also check nesting during parsing, that only checks the
        // number of recursive parse calls. It does not necessarily cover
        // all possible recursive nestings of the Hir itself. For example,
        // repetition operators don't require recursive parse calls. So one
        // can stack them arbitrarily without overflowing the stack in the
        // *parser*. But then if one recurses over the resulting Hir, a stack
        // overflow is possible. So here we check the Hir nesting level
        // thoroughly to ensure it isn't nested too deeply.
        //
        // Note that we do still need the nesting limit check in the parser as
        // well, since that will avoid overflowing the stack during parse time
        // before the complete Hir value is constructed.
        check_hir_nesting(&hir, self.config.nest_limit)?;
        Ok(hir)
    }