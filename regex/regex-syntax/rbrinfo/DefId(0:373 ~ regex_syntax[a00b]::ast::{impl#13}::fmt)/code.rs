fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::ast::print::Printer;
        Printer::new().print(self, f)
    }