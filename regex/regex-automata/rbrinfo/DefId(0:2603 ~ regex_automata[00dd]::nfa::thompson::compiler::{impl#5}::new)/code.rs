fn new() -> Utf8State {
        Utf8State { compiled: Utf8BoundedMap::new(10_000), uncompiled: vec![] }
    }