pub(crate) fn new(builder: &PikeVM) -> PikeVMCache {
        PikeVMCache(Some(builder.get().0.create_cache()))
    }