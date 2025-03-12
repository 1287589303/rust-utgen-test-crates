pub fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&L>, Pin<&R>> {
        // SAFETY: We can use `new_unchecked` because the `inner` parts are
        // guaranteed to be pinned, as they come from `self` which is pinned.
        unsafe { map_either!(Pin::get_ref(self), inner => Pin::new_unchecked(inner)) }
    }