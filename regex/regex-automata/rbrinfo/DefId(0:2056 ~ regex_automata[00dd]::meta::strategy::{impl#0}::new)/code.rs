fn new(pre: P) -> Arc<dyn Strategy> {
        // The only thing we support when we use prefilters directly as a
        // strategy is the start and end of the overall match for a single
        // pattern. In other words, exactly one implicit capturing group. Which
        // is exactly what we use here for a GroupInfo.
        let group_info = GroupInfo::new([[None::<&str>]]).unwrap();
        Arc::new(Pre { pre, group_info })
    }