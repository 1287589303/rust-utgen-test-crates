fn visit_type_param_bound_mut(&mut self, bound: &mut TypeParamBound) {
        match bound {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            TypeParamBound::Trait(bound) => self.visit_path_mut(&mut bound.path),
            TypeParamBound::Lifetime(_)
            | TypeParamBound::PreciseCapture(_)
            | TypeParamBound::Verbatim(_) => {}
            _ => {}
        }
    }