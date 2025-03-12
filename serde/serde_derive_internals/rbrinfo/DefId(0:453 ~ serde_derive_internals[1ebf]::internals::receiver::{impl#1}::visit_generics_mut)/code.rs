fn visit_generics_mut(&mut self, generics: &mut Generics) {
        for param in &mut generics.params {
            match param {
                GenericParam::Type(param) => {
                    for bound in &mut param.bounds {
                        self.visit_type_param_bound_mut(bound);
                    }
                }
                GenericParam::Lifetime(_) | GenericParam::Const(_) => {}
            }
        }
        if let Some(where_clause) = &mut generics.where_clause {
            for predicate in &mut where_clause.predicates {
                match predicate {
                    #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
                    WherePredicate::Type(predicate) => {
                        self.visit_type_mut(&mut predicate.bounded_ty);
                        for bound in &mut predicate.bounds {
                            self.visit_type_param_bound_mut(bound);
                        }
                    }
                    WherePredicate::Lifetime(_) => {}
                    _ => {}
                }
            }
        }
    }