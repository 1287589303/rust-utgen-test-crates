use core::fmt;
use crate::ast::{
    self, visitor::{self, Visitor},
    Ast,
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
    pub fn print<W: fmt::Write>(&mut self, ast: &Ast, wtr: W) -> fmt::Result {}
}
impl PrinterBuilder {
    fn new() -> PrinterBuilder {
        PrinterBuilder { _priv: () }
    }
    fn build(&self) -> Printer {
        Printer { _priv: () }
    }
}
