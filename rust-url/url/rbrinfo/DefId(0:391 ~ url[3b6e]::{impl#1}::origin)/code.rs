pub fn origin(&self) -> Origin {
        origin::url_origin(self)
    }