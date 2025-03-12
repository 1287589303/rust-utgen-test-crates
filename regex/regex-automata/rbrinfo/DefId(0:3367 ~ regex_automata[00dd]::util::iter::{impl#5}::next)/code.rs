fn next(&mut self) -> Option<HalfMatch> {
        match self.0.next()? {
            Ok(m) => Some(m),
            Err(err) => panic!(
                "unexpected regex half find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                err,
            ),
        }
    }