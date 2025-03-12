use core::fmt;
use crate::{
    hir::{
        self, visitor::{self, Visitor},
        Hir, HirKind,
    },
    is_meta_character,
};
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Clone, Debug)]
struct PrinterBuilder {
    _priv: (),
}
impl Printer {
    pub fn new() -> Printer {
        PrinterBuilder::new().build()
    }
    pub fn print<W: fmt::Write>(&mut self, hir: &Hir, wtr: W) -> fmt::Result {}
}
impl PrinterBuilder {
    fn new() -> PrinterBuilder {
        PrinterBuilder { _priv: () }
    }
    fn build(&self) -> Printer {
        Printer { _priv: () }
    }
}
