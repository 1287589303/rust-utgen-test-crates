pub fn custom_serde_path(&self) -> Option<&syn::Path> {
        self.serde_path.as_ref()
    }