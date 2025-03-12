fn get_unanchored_prefix(&self) -> bool {
        #[cfg(test)]
        {
            self.unanchored_prefix.unwrap_or(true)
        }
        #[cfg(not(test))]
        {
            true
        }
    }