pub fn serde_path(&self) -> Cow<syn::Path> {
        self.custom_serde_path()
            .map_or_else(|| Cow::Owned(parse_quote!(_serde)), Cow::Borrowed)
    }