pub fn advance<F>(&mut self, finder: F) -> Option<Match>
    where
        F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
    {
        match self.try_advance(finder) {
            Ok(m) => m,
            Err(err) => panic!(
                "unexpected regex find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                err,
            ),
        }
    }