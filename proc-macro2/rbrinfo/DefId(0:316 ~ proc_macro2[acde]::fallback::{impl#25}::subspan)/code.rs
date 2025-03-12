pub(crate) fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {
        #[cfg(not(span_locations))]
        {
            let _ = range;
            None
        }

        #[cfg(span_locations)]
        {
            use core::ops::Bound;

            let lo = match range.start_bound() {
                Bound::Included(start) => {
                    let start = u32::try_from(*start).ok()?;
                    self.span.lo.checked_add(start)?
                }
                Bound::Excluded(start) => {
                    let start = u32::try_from(*start).ok()?;
                    self.span.lo.checked_add(start)?.checked_add(1)?
                }
                Bound::Unbounded => self.span.lo,
            };
            let hi = match range.end_bound() {
                Bound::Included(end) => {
                    let end = u32::try_from(*end).ok()?;
                    self.span.lo.checked_add(end)?.checked_add(1)?
                }
                Bound::Excluded(end) => {
                    let end = u32::try_from(*end).ok()?;
                    self.span.lo.checked_add(end)?
                }
                Bound::Unbounded => self.span.hi,
            };
            if lo <= hi && hi <= self.span.hi {
                Some(Span { lo, hi })
            } else {
                None
            }
        }
    }