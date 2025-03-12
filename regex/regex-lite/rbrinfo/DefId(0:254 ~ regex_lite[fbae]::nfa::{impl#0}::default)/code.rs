fn default() -> Config {
        Config { size_limit: Some(10 * (1 << 20)) }
    }