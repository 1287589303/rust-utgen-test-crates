pub(crate) fn overwrite(&self, o: Config) -> Config {
        Config {
            utf8: o.utf8.or(self.utf8),
            reverse: o.reverse.or(self.reverse),
            nfa_size_limit: o.nfa_size_limit.or(self.nfa_size_limit),
            shrink: o.shrink.or(self.shrink),
            which_captures: o.which_captures.or(self.which_captures),
            look_matcher: o.look_matcher.or_else(|| self.look_matcher.clone()),
            #[cfg(test)]
            unanchored_prefix: o.unanchored_prefix.or(self.unanchored_prefix),
        }
    }