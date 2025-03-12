fn eq(&self, other: &T) -> bool {
        let other = other.as_ref();
        if self.raw {
            other.starts_with("r#") && *self.sym == other[2..]
        } else {
            *self.sym == *other
        }
    }