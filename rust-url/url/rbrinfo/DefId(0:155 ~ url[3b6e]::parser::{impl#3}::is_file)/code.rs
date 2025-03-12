pub fn is_file(&self) -> bool {
        matches!(*self, SchemeType::File)
    }