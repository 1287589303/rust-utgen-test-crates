use core::fmt;
use crate::ast::{
    self, visitor::{self, Visitor},
    Ast,
};
#[derive(Clone, Debug)]
struct PrinterBuilder {
    _priv: (),
}
impl PrinterBuilder {
    fn new() -> PrinterBuilder {
        PrinterBuilder { _priv: () }
    }
    fn build(&self) -> Printer {}
}
