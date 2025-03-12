use core::fmt;
use crate::ast::{
    self, visitor::{self, Visitor},
    Ast,
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
