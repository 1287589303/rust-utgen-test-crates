fn maybe_parse_posix_class(&self) -> Option<hir::Class> {
        // POSIX character classes are interesting from a parsing perspective
        // because parsing cannot fail with any interesting error. For example,
        // in order to use an POSIX character class, it must be enclosed in
        // double brackets, e.g., `[[:alnum:]]`. Alternatively, you might think
        // of it as "POSIX character classes have the syntax `[:NAME:]` which
        // can only appear within character brackets." This means that things
        // like `[[:lower:]A]` are legal constructs.
        //
        // However, if one types an incorrect POSIX character class, e.g.,
        // `[[:loower:]]`, then we treat that as if it were normal nested
        // character class containing the characters `:elorw`. (Which isn't
        // supported and results in an error in regex-lite.) One might argue
        // that we should return an error instead since the repeated colons
        // give away the intent to write an POSIX class. But what if the user
        // typed `[[:lower]]` instead? How can we tell that was intended to be
        // a POSXI class and not just a normal nested class?
        //
        // Reasonable people can probably disagree over this, but for better
        // or worse, we implement semantics that never fails at the expense of
        // better failure modes.
        assert_eq!(self.char(), '[');

        // If parsing fails, then we back up the parser to this starting point.
        let start_pos = self.pos();
        let start_char = self.char.get();
        let reset = || {
            self.pos.set(start_pos);
            self.char.set(start_char);
        };

        let mut negated = false;
        if !self.bump() || self.char() != ':' {
            reset();
            return None;
        }
        if !self.bump() {
            reset();
            return None;
        }
        if self.char() == '^' {
            negated = true;
            if !self.bump() {
                reset();
                return None;
            }
        }
        let name_start = self.pos();
        while self.char() != ':' && self.bump() {}
        if self.is_done() {
            reset();
            return None;
        }
        let name = &self.pattern()[name_start..self.pos()];
        if !self.bump_if(":]") {
            reset();
            return None;
        }
        if let Ok(ranges) = posix_class(name) {
            let mut class = hir::Class::new(ranges);
            if negated {
                class.negate();
            }
            return Some(class);
        }
        reset();
        None
    }