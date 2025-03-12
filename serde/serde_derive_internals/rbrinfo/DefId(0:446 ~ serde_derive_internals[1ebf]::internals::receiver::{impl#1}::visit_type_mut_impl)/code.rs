fn visit_type_mut_impl(&mut self, ty: &mut Type) {
        match ty {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Type::Array(ty) => {
                self.visit_type_mut(&mut ty.elem);
                self.visit_expr_mut(&mut ty.len);
            }
            Type::BareFn(ty) => {
                for arg in &mut ty.inputs {
                    self.visit_type_mut(&mut arg.ty);
                }
                self.visit_return_type_mut(&mut ty.output);
            }
            Type::Group(ty) => self.visit_type_mut(&mut ty.elem),
            Type::ImplTrait(ty) => {
                for bound in &mut ty.bounds {
                    self.visit_type_param_bound_mut(bound);
                }
            }
            Type::Macro(ty) => self.visit_macro_mut(&mut ty.mac),
            Type::Paren(ty) => self.visit_type_mut(&mut ty.elem),
            Type::Path(ty) => {
                if let Some(qself) = &mut ty.qself {
                    self.visit_type_mut(&mut qself.ty);
                }
                self.visit_path_mut(&mut ty.path);
            }
            Type::Ptr(ty) => self.visit_type_mut(&mut ty.elem),
            Type::Reference(ty) => self.visit_type_mut(&mut ty.elem),
            Type::Slice(ty) => self.visit_type_mut(&mut ty.elem),
            Type::TraitObject(ty) => {
                for bound in &mut ty.bounds {
                    self.visit_type_param_bound_mut(bound);
                }
            }
            Type::Tuple(ty) => {
                for elem in &mut ty.elems {
                    self.visit_type_mut(elem);
                }
            }

            Type::Infer(_) | Type::Never(_) | Type::Verbatim(_) => {}

            _ => {}
        }
    }