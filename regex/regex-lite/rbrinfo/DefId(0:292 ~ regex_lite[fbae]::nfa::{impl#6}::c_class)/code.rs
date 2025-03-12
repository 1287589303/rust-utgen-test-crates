fn c_class(&self, class: &hir::Class) -> Result<ThompsonRef, Error> {
        let id = if class.ranges.is_empty() {
            // Technically using an explicit fail state probably isn't
            // necessary. Because if you try to match against an empty Ranges,
            // then it should turn up with nothing regardless of input, and
            // thus "acts" like a Fail state. But it's better to be more
            // explicit, and there's no real cost to doing so.
            self.add(State::Fail)
        } else {
            let ranges =
                class.ranges.iter().map(|r| (r.start, r.end)).collect();
            self.add(State::Ranges { target: 0, ranges })
        }?;
        Ok(ThompsonRef { start: id, end: id })
    }