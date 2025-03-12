fn _new_fallback(inner: fallback::Group) -> Self {
        Group {
            inner: imp::Group::from(inner),
        }
    }