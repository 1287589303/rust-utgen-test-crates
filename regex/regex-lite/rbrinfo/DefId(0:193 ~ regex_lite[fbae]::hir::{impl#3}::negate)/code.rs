fn negate(&mut self) {
        const MIN: char = '\x00';
        const MAX: char = char::MAX;

        if self.ranges.is_empty() {
            self.ranges.push(ClassRange { start: MIN, end: MAX });
            return;
        }

        // There should be a way to do this in-place with constant memory,
        // but I couldn't figure out a simple way to do it. So just append
        // the negation to the end of this range, and then drain it before
        // we're done.
        let drain_end = self.ranges.len();

        // If our class doesn't start the minimum possible char, then negation
        // needs to include all codepoints up to the minimum in this set.
        if self.ranges[0].start > MIN {
            self.ranges.push(ClassRange {
                start: MIN,
                // OK because we know it's bigger than MIN.
                end: prev_char(self.ranges[0].start).unwrap(),
            });
        }
        for i in 1..drain_end {
            // let lower = self.ranges[i - 1].upper().increment();
            // let upper = self.ranges[i].lower().decrement();
            // self.ranges.push(I::create(lower, upper));
            self.ranges.push(ClassRange {
                // OK because we know i-1 is never the last range and therefore
                // there must be a range greater than it. It therefore follows
                // that 'end' can never be char::MAX, and thus there must be
                // a next char.
                start: next_char(self.ranges[i - 1].end).unwrap(),
                // Since 'i' is guaranteed to never be the first range, it
                // follows that there is always a range before this and thus
                // 'start' can never be '\x00'. Thus, there must be a previous
                // char.
                end: prev_char(self.ranges[i].start).unwrap(),
            });
        }
        if self.ranges[drain_end - 1].end < MAX {
            // let lower = self.ranges[drain_end - 1].upper().increment();
            // self.ranges.push(I::create(lower, I::Bound::max_value()));
            self.ranges.push(ClassRange {
                // OK because we know 'end' is less than char::MAX, and thus
                // there is a next char.
                start: next_char(self.ranges[drain_end - 1].end).unwrap(),
                end: MAX,
            });
        }
        self.ranges.drain(..drain_end);
        // We don't need to canonicalize because we processed the ranges above
        // in canonical order and the new ranges we added based on those are
        // also necessarily in canonical order.
    }