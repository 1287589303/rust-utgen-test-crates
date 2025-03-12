pub fn new(re: &Regex) -> Cache {
        re.create_cache()
    }