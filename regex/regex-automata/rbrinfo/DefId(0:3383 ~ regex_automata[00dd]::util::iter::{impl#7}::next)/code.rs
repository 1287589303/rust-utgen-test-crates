fn next(&mut self) -> Option<Result<Match, MatchError>> {
        self.it.try_advance(&mut self.finder).transpose()
    }