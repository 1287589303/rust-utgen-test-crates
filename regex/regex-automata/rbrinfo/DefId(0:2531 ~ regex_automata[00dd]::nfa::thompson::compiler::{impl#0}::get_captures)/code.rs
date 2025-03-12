pub fn get_captures(&self) -> bool {
        self.get_which_captures().is_any()
    }