fn visit_path_arguments_mut(&mut self, arguments: &mut PathArguments) {
        match arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(arguments) => {
                for arg in &mut arguments.args {
                    match arg {
                        #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
                        GenericArgument::Type(arg) => self.visit_type_mut(arg),
                        GenericArgument::AssocType(arg) => self.visit_type_mut(&mut arg.ty),
                        GenericArgument::Lifetime(_)
                        | GenericArgument::Const(_)
                        | GenericArgument::AssocConst(_)
                        | GenericArgument::Constraint(_) => {}
                        _ => {}
                    }
                }
            }
            PathArguments::Parenthesized(arguments) => {
                for argument in &mut arguments.inputs {
                    self.visit_type_mut(argument);
                }
                self.visit_return_type_mut(&mut arguments.output);
            }
        }
    }