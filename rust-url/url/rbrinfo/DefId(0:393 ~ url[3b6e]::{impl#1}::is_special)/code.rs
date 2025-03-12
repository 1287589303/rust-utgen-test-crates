pub fn is_special(&self) -> bool {
        let scheme_type = SchemeType::from(self.scheme());
        scheme_type.is_special()
    }