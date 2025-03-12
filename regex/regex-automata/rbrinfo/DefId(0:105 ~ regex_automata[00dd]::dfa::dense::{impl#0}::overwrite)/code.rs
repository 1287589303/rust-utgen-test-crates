pub(crate) fn overwrite(&self, o: Config) -> Config {
        Config {
            accelerate: o.accelerate.or(self.accelerate),
            pre: o.pre.or_else(|| self.pre.clone()),
            minimize: o.minimize.or(self.minimize),
            match_kind: o.match_kind.or(self.match_kind),
            start_kind: o.start_kind.or(self.start_kind),
            starts_for_each_pattern: o
                .starts_for_each_pattern
                .or(self.starts_for_each_pattern),
            byte_classes: o.byte_classes.or(self.byte_classes),
            unicode_word_boundary: o
                .unicode_word_boundary
                .or(self.unicode_word_boundary),
            quitset: o.quitset.or(self.quitset),
            specialize_start_states: o
                .specialize_start_states
                .or(self.specialize_start_states),
            dfa_size_limit: o.dfa_size_limit.or(self.dfa_size_limit),
            determinize_size_limit: o
                .determinize_size_limit
                .or(self.determinize_size_limit),
        }
    }