fn next(&mut self) -> Option<(alphabet::Unit, StateID)> {
        self.it.next().map(|(i, &id)| {
            let unit = if i + 1 == self.len {
                alphabet::Unit::eoi(i)
            } else {
                let b = u8::try_from(i)
                    .expect("raw byte alphabet is never exceeded");
                alphabet::Unit::u8(b)
            };
            (unit, id)
        })
    }