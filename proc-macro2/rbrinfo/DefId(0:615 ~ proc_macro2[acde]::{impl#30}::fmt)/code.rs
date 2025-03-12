fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_struct("Punct");
        debug.field("char", &self.ch);
        debug.field("spacing", &self.spacing);
        imp::debug_span_field_if_nontrivial(&mut debug, self.span.inner);
        debug.finish()
    }