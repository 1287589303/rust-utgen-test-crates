fn as_mut_string(&mut self) -> &mut String {
        &mut self.url.as_mut().unwrap().serialization
    }