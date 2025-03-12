fn parse_class_range(
        &self,
        union: &mut Vec<hir::ClassRange>,
    ) -> Result<(), Error> {
        let prim1 = self.parse_class_item()?;
        self.bump_space();
        if self.is_done() {
            return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_ITEM));
        }
        // If the next char isn't a `-`, then we don't have a range.
        // There are two exceptions. If the char after a `-` is a `]`, then
        // `-` is interpreted as a literal `-`. Alternatively, if the char
        // after a `-` is a `-`, then `--` corresponds to a "difference"
        // operation. (Which we don't support in regex-lite, but error about
        // specifically in an effort to be loud about differences between the
        // main regex crate where possible.)
        if self.char() != '-'
            || self.peek_space() == Some(']')
            || self.peek_space() == Some('-')
        {
            union.extend_from_slice(&into_class_item_ranges(prim1)?);
            return Ok(());
        }
        // OK, now we're parsing a range, so bump past the `-` and parse the
        // second half of the range.
        if !self.bump_and_bump_space() {
            return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_DASH));
        }
        let prim2 = self.parse_class_item()?;
        let range = hir::ClassRange {
            start: into_class_item_range(prim1)?,
            end: into_class_item_range(prim2)?,
        };
        if range.start > range.end {
            return Err(Error::new(ERR_CLASS_INVALID_RANGE));
        }
        union.push(range);
        Ok(())
    }