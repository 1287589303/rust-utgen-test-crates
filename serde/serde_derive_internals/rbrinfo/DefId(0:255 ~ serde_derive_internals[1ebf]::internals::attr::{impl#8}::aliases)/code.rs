pub fn aliases(&self) -> &BTreeSet<Name> {
        self.name.deserialize_aliases()
    }