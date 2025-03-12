fn poll(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        for_both!(self.as_pin_mut(), inner => inner.poll(cx))
    }