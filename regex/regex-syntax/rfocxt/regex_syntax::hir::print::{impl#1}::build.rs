use core::fmt;
use crate::{
    hir::{
        self, visitor::{self, Visitor},
        Hir, HirKind,
    },
    is_meta_character,
};
#[derive(Clone, Debug)]
struct PrinterBuilder {
    _priv: (),
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
impl PrinterBuilder {
    fn new() -> PrinterBuilder {}
    fn build(&self) -> Printer {
        Printer { _priv: () }
    }
}
