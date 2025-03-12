fn self_to_expr_path(&self, path: &mut Path) {
        let self_ty = self.self_ty(path.segments[0].ident.span());
        let variant = mem::replace(path, self_ty.path);
        for segment in &mut path.segments {
            if let PathArguments::AngleBracketed(bracketed) = &mut segment.arguments {
                if bracketed.colon2_token.is_none() && !bracketed.args.is_empty() {
                    bracketed.colon2_token = Some(<Token![::]>::default());
                }
            }
        }
        if variant.segments.len() > 1 {
            path.segments.push_punct(<Token![::]>::default());
            path.segments.extend(variant.segments.into_pairs().skip(1));
        }
    }