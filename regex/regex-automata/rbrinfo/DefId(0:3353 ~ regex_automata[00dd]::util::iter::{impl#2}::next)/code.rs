fn next(&mut self) -> Option<Result<HalfMatch, MatchError>> {
        self.it.try_advance_half(&mut self.finder).transpose()
    }