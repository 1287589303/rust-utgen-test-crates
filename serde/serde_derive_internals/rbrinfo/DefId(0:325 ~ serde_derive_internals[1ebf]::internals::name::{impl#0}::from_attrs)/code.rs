pub(crate) fn from_attrs(
        source_name: Name,
        ser_name: Attr<Name>,
        de_name: Attr<Name>,
        de_aliases: Option<VecAttr<Name>>,
    ) -> Self {
        let mut alias_set = BTreeSet::new();
        if let Some(de_aliases) = de_aliases {
            for alias_name in de_aliases.get() {
                alias_set.insert(alias_name);
            }
        }

        let ser_name = ser_name.get();
        let ser_renamed = ser_name.is_some();
        let de_name = de_name.get();
        let de_renamed = de_name.is_some();
        MultiName {
            serialize: ser_name.unwrap_or_else(|| source_name.clone()),
            serialize_renamed: ser_renamed,
            deserialize: de_name.unwrap_or(source_name),
            deserialize_renamed: de_renamed,
            deserialize_aliases: alias_set,
        }
    }