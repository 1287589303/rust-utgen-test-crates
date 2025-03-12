pub(crate) fn is_match(
        &self,
        cache: &mut PikeVMCache,
        input: &Input<'_>,
    ) -> bool {
        self.0.is_match(cache.0.as_mut().unwrap(), input.clone())
    }