pub(crate) fn minimize(&mut self) {
        Minimizer::new(self).run();
    }