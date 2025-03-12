fn put_state_builder(&mut self, builder: StateBuilderNFA) {
        let _ = core::mem::replace(
            &mut self.cache.scratch_state_builder,
            builder.clear(),
        );
    }