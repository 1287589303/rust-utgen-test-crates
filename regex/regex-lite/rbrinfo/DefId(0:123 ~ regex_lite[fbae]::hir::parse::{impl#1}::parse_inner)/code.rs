fn parse_inner(&self) -> Result<Hir, Error> {
        let depth = self.increment_depth()?;
        let mut alternates = vec![];
        let mut concat = vec![];
        loop {
            self.bump_space();
            if self.is_done() {
                break;
            }
            match self.char() {
                '(' => {
                    // Save the old flags and reset them only when we close
                    // the group.
                    let oldflags = *self.flags.borrow();
                    if let Some(sub) = self.parse_group()? {
                        concat.push(sub);
                        // We only reset them here because if 'parse_group'
                        // returns None, then that means it handled a flag
                        // directive, e.g., '(?ism)'. And the whole point is
                        // that those flags remain active until either disabled
                        // or the end of the pattern or current group.
                        *self.flags.borrow_mut() = oldflags;
                    }
                    if self.char.get() != Some(')') {
                        return Err(Error::new(ERR_UNCLOSED_GROUP));
                    }
                    self.bump();
                }
                ')' => {
                    if depth == 0 {
                        return Err(Error::new(ERR_UNOPENED_GROUP));
                    }
                    break;
                }
                '|' => {
                    alternates.push(Hir::concat(core::mem::take(&mut concat)));
                    self.bump();
                }
                '[' => concat.push(self.parse_class()?),
                '?' | '*' | '+' => {
                    concat = self.parse_uncounted_repetition(concat)?;
                }
                '{' => {
                    concat = self.parse_counted_repetition(concat)?;
                }
                _ => concat.push(self.parse_primitive()?),
            }
        }
        self.decrement_depth();
        alternates.push(Hir::concat(concat));
        // N.B. This strips off the "alternation" if there's only one branch.
        Ok(Hir::alternation(alternates))
    }