pub fn parse_scheme<'i>(&mut self, mut input: Input<'i>) -> Result<Input<'i>, ()> {
        // starts_with will also fail for empty strings so we can skip that comparison for perf
        if !input.starts_with(ascii_alpha) {
            return Err(());
        }
        debug_assert!(self.serialization.is_empty());
        while let Some(c) = input.next() {
            match c {
                'a'..='z' | '0'..='9' | '+' | '-' | '.' => self.serialization.push(c),
                'A'..='Z' => self.serialization.push(c.to_ascii_lowercase()),
                ':' => return Ok(input),
                _ => {
                    self.serialization.clear();
                    return Err(());
                }
            }
        }
        // EOF before ':'
        if self.context == Context::Setter {
            Ok(input)
        } else {
            self.serialization.clear();
            Err(())
        }
    }