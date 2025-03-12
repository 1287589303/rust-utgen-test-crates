fn build_many_bytes(&self) -> Result<crate::bytes::RegexSet, Error> {
        let metac = self
            .metac
            .clone()
            .match_kind(MatchKind::All)
            .utf8_empty(false)
            .which_captures(WhichCaptures::None);
        let syntaxc = self.syntaxc.clone().utf8(false);
        let patterns = Arc::from(self.pats.as_slice());
        meta::Builder::new()
            .configure(metac)
            .syntax(syntaxc)
            .build_many(&patterns)
            .map(|meta| crate::bytes::RegexSet { meta, patterns })
            .map_err(Error::from_meta_build_error)
    }