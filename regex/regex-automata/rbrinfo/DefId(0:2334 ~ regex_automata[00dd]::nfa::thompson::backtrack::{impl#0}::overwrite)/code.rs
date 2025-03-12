pub(crate) fn overwrite(&self, o: Config) -> Config {
        Config {
            pre: o.pre.or_else(|| self.pre.clone()),
            visited_capacity: o.visited_capacity.or(self.visited_capacity),
        }
    }