pub(crate) fn set_look_need(
        &mut self,
        set: impl FnMut(LookSet) -> LookSet,
    ) {
        self.repr_vec().set_look_need(set)
    }