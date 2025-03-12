fn visit_type_path_mut_impl(&mut self, ty: &mut TypePath) {
        if let Some(qself) = &mut ty.qself {
            self.visit_type_mut(&mut qself.ty);
        }
        self.visit_path_mut(&mut ty.path);
    }