pub(crate) fn reset(&mut self, builder: &PikeVM) {
        self.0.as_mut().unwrap().reset(&builder.get().0);
    }