fn get_state_builder(&mut self) -> StateBuilderEmpty {
        core::mem::replace(
            &mut self.scratch_state_builder,
            StateBuilderEmpty::new(),
        )
    }