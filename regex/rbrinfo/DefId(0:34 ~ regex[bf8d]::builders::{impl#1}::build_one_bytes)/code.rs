fn build_one_bytes(&self) -> Result<crate::bytes::Regex, Error> {
        assert_eq!(1, self.pats.len());
        let metac = self
            .metac
            .clone()
            .match_kind(MatchKind::LeftmostFirst)
            .utf8_empty(false);
        let syntaxc = self.syntaxc.clone().utf8(false);
        let pattern = Arc::from(self.pats[0].as_str());
        meta::Builder::new()
            .configure(metac)
            .syntax(syntaxc)
            .build(&pattern)
            .map(|meta| crate::bytes::Regex { meta, pattern })
            .map_err(Error::from_meta_build_error)
    }