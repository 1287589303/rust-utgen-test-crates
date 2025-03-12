pub fn get_look_matcher(&self) -> LookMatcher {
        self.look_matcher.clone().unwrap_or(LookMatcher::default())
    }