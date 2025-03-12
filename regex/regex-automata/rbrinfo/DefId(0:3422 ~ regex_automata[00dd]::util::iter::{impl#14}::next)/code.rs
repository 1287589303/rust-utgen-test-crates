fn next(&mut self) -> Option<Captures> {
        match self.0.next()? {
            Ok(m) => Some(m),
            Err(err) => panic!(
                "unexpected regex captures error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                err,
            ),
        }
    }