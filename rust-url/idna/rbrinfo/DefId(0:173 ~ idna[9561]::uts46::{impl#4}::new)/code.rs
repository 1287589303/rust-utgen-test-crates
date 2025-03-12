pub const fn new() -> Self {
        Self {
            data: idna_adapter::Adapter::new(),
        }
    }