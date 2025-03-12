pub fn advance_half<F>(&mut self, finder: F) -> Option<HalfMatch>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {
        match self.try_advance_half(finder) {
            Ok(m) => m,
            Err(err) => panic!(
                "unexpected regex half find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                err,
            ),
        }
    }