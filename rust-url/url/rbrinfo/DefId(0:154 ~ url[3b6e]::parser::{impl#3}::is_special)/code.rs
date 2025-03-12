pub fn is_special(&self) -> bool {
        !matches!(*self, SchemeType::NotSpecial)
    }