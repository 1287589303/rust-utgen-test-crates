fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::hir::print::Printer::new().print(self, f)
    }