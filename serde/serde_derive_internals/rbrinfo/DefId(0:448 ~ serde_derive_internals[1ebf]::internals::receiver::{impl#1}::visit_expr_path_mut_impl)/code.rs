fn visit_expr_path_mut_impl(&mut self, expr: &mut ExprPath) {
        if let Some(qself) = &mut expr.qself {
            self.visit_type_mut(&mut qself.ty);
        }
        self.visit_path_mut(&mut expr.path);
    }