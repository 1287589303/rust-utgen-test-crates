fn next(&mut self) -> Option<Match> {
        match self.0.next()? {
            Ok(m) => Some(m),
            Err(err) => panic!(
                "unexpected regex find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                err,
            ),
        }
    }