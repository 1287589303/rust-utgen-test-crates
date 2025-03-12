pub(crate) fn overwrite(&self, o: Config) -> Config {
        Config {
            match_kind: o.match_kind.or(self.match_kind),
            pre: o.pre.or_else(|| self.pre.clone()),
        }
    }