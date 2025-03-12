fn visit_data_mut(&mut self, data: &mut Data) {
        match data {
            Data::Struct(data) => {
                for field in &mut data.fields {
                    self.visit_type_mut(&mut field.ty);
                }
            }
            Data::Enum(data) => {
                for variant in &mut data.variants {
                    for field in &mut variant.fields {
                        self.visit_type_mut(&mut field.ty);
                    }
                }
            }
            Data::Union(_) => {}
        }
    }