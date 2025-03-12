fn visit_return_type_mut(&mut self, return_type: &mut ReturnType) {
        match return_type {
            ReturnType::Default => {}
            ReturnType::Type(_, output) => self.visit_type_mut(output),
        }
    }