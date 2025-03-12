fn take_inner(self) -> RcVecBuilder<TokenTree> {
        let nodrop = ManuallyDrop::new(self);
        unsafe { ptr::read(&nodrop.inner) }.make_owned()
    }