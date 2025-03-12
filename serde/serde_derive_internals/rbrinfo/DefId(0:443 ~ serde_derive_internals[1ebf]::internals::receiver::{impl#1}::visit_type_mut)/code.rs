fn visit_type_mut(&mut self, ty: &mut Type) {
        let span = if let Type::Path(node) = ty {
            if node.qself.is_none() && node.path.is_ident("Self") {
                node.path.segments[0].ident.span()
            } else {
                self.visit_type_path_mut(node);
                return;
            }
        } else {
            self.visit_type_mut_impl(ty);
            return;
        };
        *ty = Type::Path(self.self_ty(span));
    }