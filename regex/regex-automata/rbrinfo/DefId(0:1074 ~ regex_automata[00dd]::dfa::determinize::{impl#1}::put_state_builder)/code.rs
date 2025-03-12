fn put_state_builder(&mut self, builder: StateBuilderNFA) {
        let _ = core::mem::replace(
            &mut self.scratch_state_builder,
            builder.clear(),
        );
    }