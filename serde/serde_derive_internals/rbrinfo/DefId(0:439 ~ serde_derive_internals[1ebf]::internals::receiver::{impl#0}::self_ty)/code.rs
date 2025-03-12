fn self_ty(&self, span: Span) -> TypePath {
        let tokens = self.0.to_token_stream();
        let respanned = respan(tokens, span);
        syn::parse2(respanned).unwrap()
    }