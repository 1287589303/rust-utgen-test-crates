pub fn getter(&self) -> Option<&syn::ExprPath> {
        self.getter.as_ref()
    }