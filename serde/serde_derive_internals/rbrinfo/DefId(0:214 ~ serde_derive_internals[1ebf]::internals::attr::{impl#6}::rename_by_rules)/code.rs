pub fn rename_by_rules(&mut self, rules: RenameAllRules) {
        if !self.name.serialize_renamed {
            self.name.serialize.value =
                rules.serialize.apply_to_variant(&self.name.serialize.value);
        }
        if !self.name.deserialize_renamed {
            self.name.deserialize.value = rules
                .deserialize
                .apply_to_variant(&self.name.deserialize.value);
        }
        self.name
            .deserialize_aliases
            .insert(self.name.deserialize.clone());
    }