pub fn never_match() -> NFA {
        // This always succeeds because it only requires one NFA state, which
        // will never exhaust any (default) limits.
        let mut builder = Builder::new();
        let sid = builder.add_fail().unwrap();
        builder.build(sid, sid).unwrap()
    }