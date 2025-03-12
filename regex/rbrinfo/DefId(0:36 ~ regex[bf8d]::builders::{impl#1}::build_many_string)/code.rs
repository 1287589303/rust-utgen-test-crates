fn build_many_string(&self) -> Result<crate::RegexSet, Error> {
        let metac = self
            .metac
            .clone()
            .match_kind(MatchKind::All)
            .utf8_empty(true)
            .which_captures(WhichCaptures::None);
        let syntaxc = self.syntaxc.clone().utf8(true);
        let patterns = Arc::from(self.pats.as_slice());
        meta::Builder::new()
            .configure(metac)
            .syntax(syntaxc)
            .build_many(&patterns)
            .map(|meta| crate::RegexSet { meta, patterns })
            .map_err(Error::from_meta_build_error)
    }